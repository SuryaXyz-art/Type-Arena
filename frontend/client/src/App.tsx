import { useState, useEffect } from 'react';
import io from 'socket.io-client';
import { TypingArea } from './components/TypingArea';
import { lineraService } from './services/LineraService';
import { walletService } from './services/WalletService';
import type { Room, Tournament, LeaderboardEntry, Config } from './types';
import type { Client } from '@linera/client';

const socket = io('http://localhost:3001');

type GameState = 'menu' | 'lobby' | 'countdown' | 'racing' | 'finished' | 'tournament_lobby';

function App() {
  const [gameState, setGameState] = useState<GameState>('menu');
  const [username, setUsername] = useState('');
  const [roomIdInput, setRoomIdInput] = useState('');
  const [room, setRoom] = useState<Room | null>(null);
  const [tournament, setTournament] = useState<Tournament | null>(null);
  const [leaderboard, setLeaderboard] = useState<LeaderboardEntry[]>([]);
  const [countdown, setCountdown] = useState<number | null>(null);
  const [startTime, setStartTime] = useState<number>(0);
  const [useWeb3, setUseWeb3] = useState(false);
  const [lineraClient, setLineraClient] = useState<Client | null>(null);
  const [walletAddress, setWalletAddress] = useState<string | null>(null);
  const [config, setConfig] = useState<Config | null>(null);

  useEffect(() => {
    fetch('/config.json')
      .then((response) => response.json())
      .then((data) => setConfig(data));
  }, []);

  useEffect(() => {
    if (useWeb3 && lineraClient && config) {
      lineraService.setClient(lineraClient);
      lineraService.setAppIds(config.tokenAppId, config.marketAppId, config.oracleAppId);
    }
  }, [useWeb3, lineraClient, config]);

  useEffect(() => {
    socket.on('room_created', (room: Room) => {
      setRoom(room);
      setGameState('lobby');
      if (useWeb3) {
        lineraService.createRoom(room.id, room.text).catch(console.error);
      }
    });

    socket.on('room_joined', (room: Room) => {
      setRoom(room);
      setGameState('lobby');
    });

    socket.on('player_joined', (room: Room) => {
      setRoom(room);
    });

    socket.on('race_starting', ({ countdown }: { countdown: number }) => {
      setGameState('countdown');
      let count = countdown / 1000;
      setCountdown(count);
      const interval = setInterval(() => {
        count--;
        setCountdown(count);
        if (count <= 0) clearInterval(interval);
      }, 1000);
    });

    socket.on('race_started', ({ startTime, text }: { startTime: number, text: string }) => {
      setRoom(prev => prev ? ({ ...prev, text }) : null);
      setStartTime(startTime);
      setGameState('racing');
    });

    socket.on('room_update', (updatedRoom: Room) => {
      setRoom(updatedRoom);
    });

    socket.on('race_finished', (finalRoom: Room) => {
      setRoom(finalRoom);
      setGameState('finished');

      if (useWeb3) {
        // Submit own score
        const myPlayer = finalRoom.players.find((p) => p.id === socket.id);
        if (myPlayer) {
          lineraService.submitScore(finalRoom.id, myPlayer.wpm, Date.now() - startTime).catch(console.error);
        }

        // Host finishes the room on-chain
        if (finalRoom.hostId === socket.id) {
          setTimeout(() => {
            lineraService.finishRoom(finalRoom.id).catch(console.error);
          }, 2000);
        }
      }
    });

    // Tournament Events
    socket.on('tournament_created', (t: Tournament) => {
      setTournament(t);
      setGameState('tournament_lobby');
    });

    socket.on('tournament_joined', (t: Tournament) => {
      setTournament(t);
      setGameState('tournament_lobby');
    });

    socket.on('tournament_updated', (t: Tournament) => {
      setTournament(t);
    });

    socket.on('tournament_starting', ({ countdown }: { countdown: number }) => {
      let count = countdown / 1000;
      setCountdown(count);
      // We reuse the 'countdown' state but stay in tournament_lobby for overlay
      // Or we can add a new state 'tournament_countdown' if needed.
      // For simplicity, let's just use the countdown variable and show it in the lobby.
      const interval = setInterval(() => {
        count--;
        setCountdown(count);
        if (count <= 0) {
          clearInterval(interval);
          setCountdown(null);
        }
      }, 1000);
    });

    // Leaderboard
    socket.on('leaderboard_update', (scores: LeaderboardEntry[]) => {
      setLeaderboard(scores);
    });

    return () => {
      socket.off('room_created');
      socket.off('room_joined');
      socket.off('player_joined');
      socket.off('race_starting');
      socket.off('race_started');
      socket.off('room_update');
      socket.off('race_finished');
      socket.off('tournament_created');
      socket.off('tournament_joined');
      socket.off('tournament_updated');
      socket.off('tournament_starting');
      socket.off('leaderboard_update');
    };
  }, [useWeb3, startTime]);

  const handleCreate = () => {
    if (!username) return alert("Enter username");
    socket.emit('create_room', { username });
  };

  const handleJoin = () => {
    if (!username || !roomIdInput) return alert("Enter username and room ID");
    socket.emit('join_room', { roomId: roomIdInput, username });
  };

  const handleCreateTournament = () => {
    if (!username) return alert("Enter username");
    socket.emit('create_tournament', { username });
  }

  const handleJoinTournament = () => {
    if (!username || !roomIdInput) return alert("Enter username and tournament ID");
    socket.emit('join_tournament', { tournamentId: roomIdInput, username });
  }

  const handleStart = () => {
    if (room) socket.emit('start_race', { roomId: room.id });
  };

  const handleProgress = (progress: number, wpm: number) => {
    if (room) {
      socket.emit('update_progress', { roomId: room.id, progress, wpm });
    }
  };

  const handleConnectWallet = async () => {
    if (!config) {
      return alert('Config not loaded yet');
    }
    try {
      const signer = await walletService.connect();
      const address = await signer.address();
      setWalletAddress(address);
      const faucetUrl = import.meta.env.VITE_GRAPHQL_ENDPOINT || "http://localhost:8080";
      const client = await walletService.createClient(faucetUrl, config.chainId);
      setLineraClient(client);
    } catch (error) {
      console.error(error);
      alert('Failed to connect to wallet');
    }
  };

  return (
    <div className="min-h-screen bg-gray-900 text-white font-sans selection:bg-purple-500 selection:text-white">
      <header className="p-4 border-b border-gray-800 flex justify-between items-center bg-gray-900/50 backdrop-blur-md sticky top-0 z-10">
        <h1 className="text-3xl font-black bg-clip-text text-transparent bg-gradient-to-r from-purple-400 to-pink-600 italic tracking-tighter">
          TYPE<span className="text-white not-italic">ARENA</span>
        </h1>
        <div className="flex items-center gap-4">
          {walletAddress ? (
            <div className="flex items-center gap-2 mr-4">
              <span className={`text-xs font-bold ${useWeb3 ? 'text-green-400' : 'text-gray-500'}`}>WEB3 MODE</span>
              <button
                onClick={() => setUseWeb3(!useWeb3)}
                className={`w-12 h-6 rounded-full p-1 transition-colors ${useWeb3 ? 'bg-green-600' : 'bg-gray-700'}`}
              >
                <div className={`w-4 h-4 bg-white rounded-full transition-transform ${useWeb3 ? 'translate-x-6' : 'translate-x-0'}`} />
              </button>
            </div>
          ) : (
            <button
              onClick={handleConnectWallet}
              disabled={!config}
              className="px-4 py-2 bg-purple-600 hover:bg-purple-500 rounded-lg font-bold transition-all hover:scale-105 disabled:bg-gray-500"
            >
              {config ? 'Connect Wallet' : 'Loading...'}
            </button>
          )}
          {room && <span className="font-mono bg-gray-800 px-3 py-1 rounded">Room: {room.id}</span>}
          <div className={`w-3 h-3 rounded-full ${socket.connected ? 'bg-green-500' : 'bg-red-500'} shadow-[0_0_10px_rgba(34,197,94,0.5)]`} />
        </div>
      </header>

      <main className="container mx-auto p-4 md:p-8">
        {gameState === 'menu' && (
          <div className="flex flex-col md:flex-row items-start justify-center min-h-[60vh] gap-8 animate-fade-in-up">
            <div className="bg-gray-800 p-8 rounded-2xl shadow-2xl border border-gray-700 w-full max-w-md">
              <h2 className="text-2xl font-bold mb-6 text-center">Enter the Arena</h2>
              <input
                className="w-full bg-gray-900 p-4 rounded-lg mb-4 text-center text-xl font-bold focus:ring-2 focus:ring-purple-500 outline-none"
                placeholder="Choose Username"
                value={username}
                onChange={e => setUsername(e.target.value)}
              />

              <div className="grid grid-cols-2 gap-4 mb-4">
                <button
                  onClick={handleCreate}
                  className="w-full py-4 bg-gradient-to-r from-purple-600 to-blue-600 rounded-lg font-bold text-lg hover:opacity-90 transition-transform active:scale-95"
                >
                  Race Mode
                </button>
                <button
                  onClick={handleCreateTournament}
                  className="w-full py-4 bg-gradient-to-r from-yellow-600 to-orange-600 rounded-lg font-bold text-lg hover:opacity-90 transition-transform active:scale-95"
                >
                  Tournament
                </button>
              </div>

              <div className="flex gap-2">
                <input
                  className="bg-gray-900 p-4 rounded-lg flex-1 text-center font-mono uppercase focus:ring-2 focus:ring-purple-500 outline-none"
                  placeholder="CODE"
                  value={roomIdInput}
                  onChange={e => setRoomIdInput(e.target.value.toUpperCase())}
                />
                <div className="flex flex-col gap-1">
                  <button
                    onClick={handleJoin}
                    className="px-4 py-1 bg-gray-700 hover:bg-gray-600 rounded text-sm font-bold transition-colors"
                  >
                    Join Room
                  </button>
                  <button
                    onClick={handleJoinTournament}
                    className="px-4 py-1 bg-gray-700 hover:bg-gray-600 rounded text-sm font-bold transition-colors"
                  >
                    Join Tourn
                  </button>

                </div>

              </div>
            </div>

            {/* Leaderboard Panel */}
            <div className="bg-gray-800 p-6 rounded-2xl shadow-2xl border border-gray-700 w-full max-w-sm h-full max-h-[500px] overflow-hidden flex flex-col">
              <h3 className="text-xl font-bold mb-4 text-yellow-400 flex items-center gap-2">
                <span>🏆</span> Global Leaderboard
              </h3>
              <div className="overflow-y-auto flex-1 pr-2">
                <table className="w-full text-left">
                  <thead>
                    <tr className="text-gray-500 border-b border-gray-700 text-sm">
                      <th className="pb-2">Rank</th>
                      <th className="pb-2">Player</th>
                      <th className="pb-2 text-right">WPM</th>
                    </tr>
                  </thead>
                  <tbody>
                    {leaderboard.map((score, i) => (
                      <tr key={i} className="border-b border-gray-700/50 last:border-0 hover:bg-gray-700/30">
                        <td className="py-2 text-gray-400 font-mono">#{i + 1}</td>
                        <td className="py-2 font-bold max-w-[120px] truncate">{score.username}</td>
                        <td className="py-2 text-right text-green-400 font-mono">{score.wpm}</td>
                      </tr>
                    ))}
                    {leaderboard.length === 0 && (
                      <tr>
                        <td colSpan={3} className="py-2 text-center text-gray-500 italic">No records yet.</td>
                      </tr>
                    )}
                  </tbody>
                </table>
              </div>
            </div>
          </div>
        )}

        {gameState === 'tournament_lobby' && tournament && (
          <div className="max-w-4xl mx-auto text-center animate-fade-in-up relative">
            {countdown !== null && countdown > 0 && (
              <div className="absolute inset-0 flex items-center justify-center z-50 bg-gray-900/80 backdrop-blur rounded-xl">
                <div className="text-9xl font-black text-transparent bg-clip-text bg-gradient-to-br from-yellow-400 to-red-600 animate-pulse drop-shadow-[0_0_20px_rgba(255,0,0,0.5)]">
                  {countdown}
                </div>
              </div>
            )}
            <h2 className="text-3xl font-bold mb-4">Tournament Lobby</h2>
            <p className="mb-4">ID: <span className="font-mono bg-gray-800 px-2 rounded select-all">{tournament.id}</span></p>

            {tournament.status === 'waiting' && (
              <div className="mb-8">
                <p className="text-yellow-400 mb-4">Waiting for players... ({tournament.players.length}/{tournament.maxPlayers})</p>
                {tournament.hostId === socket.id && (
                  <button
                    onClick={() => socket.emit('start_tournament', { tournamentId: tournament.id })}
                    className="px-8 py-3 bg-green-600 hover:bg-green-500 rounded-lg font-bold transition-all hover:scale-105"
                  >
                    Start Tournament
                  </button>
                )}
              </div>
            )}

            <div className="bg-gray-800 rounded-xl p-6 mt-8 shadow-2xl">
              <h3 className="text-xl font-bold mb-6 text-purple-400">BRACKET</h3>
              <div className="flex gap-8 overflow-x-auto pb-4 justify-center">
                {tournament.bracket.map((round: any[], rIndex: number) => (
                  <div key={rIndex} className="flex flex-col gap-4 min-w-[200px]">
                    <div className="text-gray-400 font-bold uppercase tracking-widest text-sm mb-2">Round {rIndex + 1}</div>
                    {round.map((match: any, mIndex: number) => {
                      const isMyMatch = match.player1 === socket.id || match.player2 === socket.id;
                      const p1Name = tournament.playerNames[match.player1] || (match.player1 ? 'Unknown' : 'Waiting...');
                      const p2Name = tournament.playerNames[match.player2] || (match.player2 ? 'Unknown' : 'Waiting...');

                      return (
                        <div key={mIndex} className={`bg-gray-700 p-4 rounded-lg border-2 ${isMyMatch ? 'border-purple-500' : 'border-transparent'} relative`}>
                          <div className={`p-2 rounded ${match.winner === match.player1 ? 'bg-green-600/50' : ''} mb-2`}>{p1Name}</div>
                          <div className="text-xs text-gray-500 font-bold mb-2">VS</div>
                          <div className={`p-2 rounded ${match.winner === match.player2 ? 'bg-green-600/50' : ''}`}>{p2Name}</div>

                          {isMyMatch && match.roomId && !match.winner && (
                            <button
                              onClick={() => {
                                setRoomIdInput(match.roomId);
                                socket.emit('join_room', { roomId: match.roomId, username });
                              }}
                              className="mt-2 w-full py-1 bg-purple-600 hover:bg-purple-500 rounded font-bold text-sm animate-pulse"
                            >
                              JOIN MATCH
                            </button>
                          )}
                        </div>
                      );
                    })}
                  </div>
                ))}
              </div>
            </div>

            <button onClick={() => window.location.reload()} className="mt-8 text-gray-400 hover:text-white underline">Leave Tournament</button>
          </div>
        )}

        {(gameState === 'lobby' || gameState === 'countdown') && room && (
          <div className="max-w-2xl mx-auto">
            <div className="text-center mb-8 relative">
              {gameState === 'countdown' && (
                <div className="absolute inset-0 flex items-center justify-center z-50">
                  <div className="text-9xl font-black text-transparent bg-clip-text bg-gradient-to-br from-yellow-400 to-red-600 animate-pulse drop-shadow-[0_0_20px_rgba(255,0,0,0.5)]">
                    {countdown}
                  </div>
                </div>
              )}
              <h2 className="text-3xl font-bold mb-2">Lobby</h2>
              <p className="text-gray-400">Invite Code: <span className="text-white font-mono bg-gray-800 px-2 py-1 rounded select-all">{room.id}</span></p>
            </div>

            <div className="bg-gray-800 rounded-xl overflow-hidden shadow-xl mb-8">
              <div className="p-4 bg-gray-750 border-b border-gray-700 flex justify-between items-center">
                <span className="font-bold text-gray-400 uppercase text-sm tracking-wider">Players ({room.players.length}/25)</span>
                {room.hostId === socket.id && gameState === 'lobby' && (
                  <button onClick={handleStart} className="bg-green-500 hover:bg-green-600 text-white px-6 py-2 rounded-lg font-bold shadow-lg shadow-green-900/20 transition-all hover:scale-105">
                    Start Race
                  </button>
                )}
              </div>
              <ul>
                {room.players.map((p: any) => (
                  <li key={p.id} className="p-4 border-b border-gray-700/50 flex justify-between items-center last:border-0 hover:bg-gray-700/30 transition-colors">
                    <div className="flex items-center gap-3">
                      <div className="w-8 h-8 rounded-full bg-gradient-to-tr from-purple-500 to-blue-500 flex items-center justify-center font-bold">
                        {p.username[0].toUpperCase()}
                      </div>
                      <span className="font-semibold">{p.username}</span>
                    </div>
                    {p.id === room.hostId && <span className="text-xs bg-yellow-500/20 text-yellow-300 px-2 py-1 rounded border border-yellow-500/30">HOST</span>}
                  </li>
                ))}
              </ul>
            </div>
          </div>
        )}

        {gameState === 'racing' && room && (
          <div className="space-y-8">
            {/* Progress Bars */}
            <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
              {room.players.map((p: any) => (
                <div key={p.id} className={`bg-gray-800 p-3 rounded-lg border ${p.id === socket.id ? 'border-purple-500' : 'border-gray-700'}`}>
                  <div className="flex justify-between text-sm mb-1">
                    <span className="font-bold truncate">{p.username} <span className="text-gray-500 font-normal">({p.wpm} WPM)</span></span>
                    {p.finished && <span className="text-green-400 font-bold">FINISHED</span>}
                  </div>
                  <div className="h-2 bg-gray-700 rounded-full overflow-hidden">
                    <div
                      className="h-full bg-gradient-to-r from-blue-500 to-purple-500 transition-all duration-300 ease-out"
                      style={{ width: `${p.progress}%` }}
                    />
                  </div>
                </div>
              ))}
            </div>

            <TypingArea
              text={room.text}
              startTime={startTime}
              onProgress={handleProgress}
            />
          </div>
        )}

        {gameState === 'finished' && room && (
          <div className="max-w-2xl mx-auto text-center animate-fade-in-up">
            <h2 className="text-4xl font-extrabold mb-8 bg-clip-text text-transparent bg-gradient-to-r from-yellow-400 to-orange-500">Race Finished!</h2>

            <div className="bg-gray-800 rounded-xl overflow-hidden shadow-2xl">
              <div className="grid grid-cols-4 bg-gray-700 p-4 font-bold text-gray-300">
                <div className="col-span-1">Rank</div>
                <div className="col-span-2 text-left">Player</div>
                <div className="col-span-1">WPM</div>
              </div>
              {room.players
                .sort((a: any, b: any) => (b.wpm - a.wpm)) // Simple sort by WPM for now
                .map((p: any, i: number) => (
                  <div key={p.id} className="grid grid-cols-4 p-4 border-b border-gray-700 items-center hover:bg-gray-700/50">
                    <div className="col-span-1 text-2xl font-black text-gray-500 font-mono">
                      {i === 0 ? '🥇' : i === 1 ? '🥈' : i === 2 ? '🥉' : `#${i + 1}`}
                    </div>
                    <div className="col-span-2 text-left font-bold text-lg flex items-center gap-2">
                      {p.username}
                      {p.id === socket.id && <span className="text-xs bg-purple-500 px-2 py-0.5 rounded text-white font-normal">YOU</span>}
                    </div>
                    <div className="col-span-1 text-green-400 font-mono text-xl">{p.wpm}</div>
                  </div>
                ))}
            </div>

            <div className="mt-8">
              <button onClick={() => window.location.reload()} className="px-8 py-3 bg-gray-700 hover:bg-gray-600 rounded-lg font-bold transition-colors">
                Back to Menu
              </button>
            </div>
          </div>
        )}
      </main>
    </div>
  );
}

export default App;

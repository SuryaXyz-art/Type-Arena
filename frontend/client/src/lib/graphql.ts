import { GraphQLClient } from 'graphql-request';

// Support Vite environment variables (VITE_ prefix)
// Point directly to the Market application
export const CHAIN_ID = 'ff869722e5434effbdcb533eae9979085f0ee8283aa711a9c2501838683ff54f';
const MARKET_APP_ID = '83a0af207595ecd9242ef183ea236a1c6715d918bed8ba14e11680351b9e2d22';
const ENDPOINT = import.meta.env.VITE_GRAPHQL_ENDPOINT || `http://localhost:8080/chains/${CHAIN_ID}/applications/${MARKET_APP_ID}`;

export const client = new GraphQLClient(ENDPOINT);

// Query to get all markets (returns keys and values)
export const GET_ALL_MARKETS = `
  query GetAllMarkets {
    markets {
      keys
      entries {
        key
        value {
          id
          creator
          marketType
          durationMicros
          createdAt
          closesAt
          status
          totalPool
          upPool
          downPool
          outcome
          resolvedAt
        }
      }
    }
  }
`;

// Query to get balance - Note: In SDK 0.15.6, we need to query the Token app directly
export const GET_BALANCE = `
  query GetBalance {
    accounts {
      keys
    }
  }
`;

// Mutation to place a bet
export const PLACE_BET = `
  mutation PlaceBet($marketId: String!, $prediction: String!, $amount: String!) {
    placeBet(marketId: $marketId, prediction: $prediction, amount: $amount)
  }
`;

// Mutation to create a market
export const CREATE_MARKET = `
  mutation CreateMarket($marketType: String!, $durationMinutes: Int!) {
    createMarket(marketType: $marketType, durationMinutes: $durationMinutes)
  }
`;

// Query to get chains info
export const GET_CHAINS = `
  query GetChains {
    nextMarketId
  }
`;


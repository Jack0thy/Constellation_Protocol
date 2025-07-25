import express from 'express';
import fetch from 'node-fetch';
import cors from 'cors';

const router = express.Router();

// Enable CORS for the proxy routes
router.use(cors());

// Blockscout proxy
router.get('/blockscout/*', async (req, res) => {
  try {
    const path = req.path.replace('/api/proxy/blockscout', '');
    const url = `https://base-sepolia.blockscout.com/api/v2${path}`;
    
    const response = await fetch(url, {
      headers: {
        'Accept': 'application/json',
        'Content-Type': 'application/json'
      }
    });

    if (!response.ok) {
      throw new Error(`Blockscout API error: ${response.status}`);
    }

    const data = await response.json();
    res.json(data);
  } catch (error) {
    console.error('Proxy error:', error);
    res.status(500).json({ error: 'Failed to fetch data from Blockscout' });
  }
});

// OpenSea proxy
router.get('/opensea/*', async (req, res) => {
  try {
    const path = req.path.replace('/api/proxy/opensea', '');
    const url = `https://testnets-api.opensea.io/v2${path}`;
    
    const response = await fetch(url, {
      headers: {
        'Accept': 'application/json',
        'Content-Type': 'application/json'
      }
    });

    if (!response.ok) {
      throw new Error(`OpenSea API error: ${response.status}`);
    }

    const data = await response.json();
    res.json(data);
  } catch (error) {
    console.error('Proxy error:', error);
    res.status(500).json({ error: 'Failed to fetch data from OpenSea' });
  }
});

export default router; 
const path = require("path");

module.exports = {
  networks: {
    development: {
      host: "127.0.0.1", // Localhost (default: none)
      port: 8545, // Standard Ethereum port (default: none)
      network_id: "*", // Any network (default: none)
    },
    'dev.fifs': {
      host: "localhost",
      port: 8545,
      network_id: "*" // Match any network id
    },
    'dev.auction': {
      host: "localhost",
      port: 8545,
      network_id: "*" // Match any network id
    }
  },
  contracts_build_directory: path.join(__dirname, "../client/src/contracts")
};

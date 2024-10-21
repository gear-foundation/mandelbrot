import { HardhatUserConfig } from "hardhat/config";
import "@nomicfoundation/hardhat-toolbox";
import * as dotenv from "dotenv";

dotenv.config();

const config: HardhatUserConfig = {
  solidity: "0.8.27",
  networks: {
    hardhat: {
      chainId: 1337,
    },
    holsky: {
      url: process.env.HOLSKY_RPC_URL || "",
      accounts: process.env.HOLSKY_PRIVATE_KEY !== undefined ? [process.env.HOLSKY_PRIVATE_KEY] : [],
    },
  },
};

export default config;
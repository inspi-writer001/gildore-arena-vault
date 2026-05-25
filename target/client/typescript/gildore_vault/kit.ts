import { type Address, address, AccountRole, type IInstruction, getAddressCodec } from "@solana/kit";
import { getArrayCodec, getBooleanCodec, getStructCodec, getU16Codec, getU32Codec, getU64Codec, getU8Codec } from "@solana/codecs";
import type { Codec } from "@solana/codecs";

function getDynVecCodec<TFrom, TTo extends TFrom = TFrom>(
  itemCodec: Codec<TFrom, TTo>,
) {
  return getArrayCodec(itemCodec, { size: getU32Codec() });
}

function matchDisc(data: Uint8Array, disc: Uint8Array): boolean {
  if (data.length < disc.length) return false;
  for (let i = 0; i < disc.length; i++) {
    if (data[i] !== disc[i]) return false;
  }
  return true;
}

/* Constants */
export const PROGRAM_ADDRESS = address("2um3F4vyQwcuhwrGdPHGMwwK5C4K5rFU84cxHPoYNMKg");
export const USER_STATE_DISCRIMINATOR = new Uint8Array([1]);
export const GLOBAL_STATE_DISCRIMINATOR = new Uint8Array([2]);
export const AGENT_MARKET_DISCRIMINATOR = new Uint8Array([3]);
export const I_AGENT_DISCRIMINATOR = new Uint8Array([4]);
export const TICKER_DISCRIMINATOR = new Uint8Array([5]);
export const INITIALIZE_INSTRUCTION_DISCRIMINATOR = new Uint8Array([0]);
export const REGISTER_AGENT_INSTRUCTION_DISCRIMINATOR = new Uint8Array([1]);
export const DELETE_AGENT_INSTRUCTION_DISCRIMINATOR = new Uint8Array([2]);
export const DEPOSIT_FOR_AGENT_USE_INSTRUCTION_DISCRIMINATOR = new Uint8Array([3]);
export const REGISTER_TICKER_FOR_ME_INSTRUCTION_DISCRIMINATOR = new Uint8Array([4]);

/* Interfaces */
export interface UserState {
  userAddress: Address;
  agentId: Address;
  tickerId: Address;
  isInitialized: PodBool;
  modifiedTime: bigint;
  createdTime: bigint;
  amount: bigint;
  bump: number;
}

export interface GlobalState {
  feeDestination: Address;
  feeBps: number;
  maxFee: bigint;
  bump: number;
  admin: Array<Address>;
}

export interface AgentMarket {
  agentId: Address;
  tickerId: Address;
  isTrading: boolean;
  bump: number;
}

export interface IAgent {
  agentId: Address;
  bump: number;
  seeds: [u8];
}

export interface Ticker {
  amountToSpend: bigint;
}

export interface InitializeInstructionArgs {
  args: InitializeArgs;
}

export interface RegisterAgentInstructionArgs {
  name: PodString;
}

export interface DeleteAgentInstructionArgs {
  id: Address;
}

export interface DepositForAgentUseInstructionArgs {
  amount: bigint;
}

export interface RegisterTickerForMeInstructionArgs {
  amountToSpend: bigint;
}

export interface InitializeInstructionInput {
  payer: Address;
  globalStateAccount: Address;
  destinationTokenAccount: Address;
  systemProgram: Address;
  args: InitializeArgs;
}

export interface RegisterAgentInstructionInput {
  admin: Address;
  agent: Address;
  globalStateAccount: Address;
  systemProgram: Address;
  name: PodString;
}

export interface DeleteAgentInstructionInput {
  admin: Address;
  agent: Address;
  globalStateAccount: Address;
  systemProgram: Address;
  id: Address;
}

export interface DepositForAgentUseInstructionInput {
  payer: Address;
  user: Address;
  agent: Address;
  globalStateAccount: Address;
  userState: Address;
  userStateVault: Address;
  ticker: Address;
  destinationFeeTokenAccount: Address;
  userTokenAccount: Address;
  mint: Address;
  tokenProgram: Address;
  systemProgram: Address;
  amount: bigint;
}

export interface RegisterTickerForMeInstructionInput {
  payer: Address;
  user: Address;
  agent: Address;
  userState: Address;
  userStateVault: Address;
  ticker: Address;
  mint: Address;
  tokenProgram: Address;
  systemProgram: Address;
  amountToSpend: bigint;
}

/* Codecs */
export const UserStateCodec = getStructCodec([
  ["userAddress", getAddressCodec()],
  ["agentId", getAddressCodec()],
  ["tickerId", getAddressCodec()],
  ["isInitialized", PodBoolCodec],
  ["modifiedTime", getU64Codec()],
  ["createdTime", getU64Codec()],
  ["amount", getU64Codec()],
  ["bump", getU8Codec()],
]);

export const GlobalStateCodec = getStructCodec([
  ["feeDestination", getAddressCodec()],
  ["feeBps", getU16Codec()],
  ["maxFee", getU64Codec()],
  ["bump", getU8Codec()],
  ["admin", getDynVecCodec(getAddressCodec())],
]);

export const AgentMarketCodec = getStructCodec([
  ["agentId", getAddressCodec()],
  ["tickerId", getAddressCodec()],
  ["isTrading", getBooleanCodec()],
  ["bump", getU8Codec()],
]);

export const IAgentCodec = getStructCodec([
  ["agentId", getAddressCodec()],
  ["bump", getU8Codec()],
  ["seeds", [u8]Codec],
]);

export const TickerCodec = getStructCodec([
  ["amountToSpend", getU64Codec()],
]);

/* Enums */
export enum ProgramInstruction {
  Initialize = "Initialize",
  RegisterAgent = "RegisterAgent",
  DeleteAgent = "DeleteAgent",
  DepositForAgentUse = "DepositForAgentUse",
  RegisterTickerForMe = "RegisterTickerForMe",
}

export type DecodedInstruction =
  | { type: ProgramInstruction.Initialize; args: InitializeInstructionArgs }
  | { type: ProgramInstruction.RegisterAgent; args: RegisterAgentInstructionArgs }
  | { type: ProgramInstruction.DeleteAgent; args: DeleteAgentInstructionArgs }
  | { type: ProgramInstruction.DepositForAgentUse; args: DepositForAgentUseInstructionArgs }
  | { type: ProgramInstruction.RegisterTickerForMe; args: RegisterTickerForMeInstructionArgs };

/* Client */
export class GildoreVaultClient {

  decodeUserState(data: Uint8Array): UserState {
    if (!matchDisc(data, USER_STATE_DISCRIMINATOR)) throw new Error("Invalid UserState discriminator");
    return UserStateCodec.decode(data.slice(USER_STATE_DISCRIMINATOR.length));
  }

  decodeGlobalState(data: Uint8Array): GlobalState {
    if (!matchDisc(data, GLOBAL_STATE_DISCRIMINATOR)) throw new Error("Invalid GlobalState discriminator");
    return GlobalStateCodec.decode(data.slice(GLOBAL_STATE_DISCRIMINATOR.length));
  }

  decodeAgentMarket(data: Uint8Array): AgentMarket {
    if (!matchDisc(data, AGENT_MARKET_DISCRIMINATOR)) throw new Error("Invalid AgentMarket discriminator");
    return AgentMarketCodec.decode(data.slice(AGENT_MARKET_DISCRIMINATOR.length));
  }

  decodeIAgent(data: Uint8Array): IAgent {
    if (!matchDisc(data, I_AGENT_DISCRIMINATOR)) throw new Error("Invalid IAgent discriminator");
    return IAgentCodec.decode(data.slice(I_AGENT_DISCRIMINATOR.length));
  }

  decodeTicker(data: Uint8Array): Ticker {
    if (!matchDisc(data, TICKER_DISCRIMINATOR)) throw new Error("Invalid Ticker discriminator");
    return TickerCodec.decode(data.slice(TICKER_DISCRIMINATOR.length));
  }

  decodeInstruction(data: Uint8Array): DecodedInstruction | null {
    if (matchDisc(data, INITIALIZE_INSTRUCTION_DISCRIMINATOR)) {
      const argsCodec = getStructCodec([
        ["args", InitializeArgsCodec],
      ]);
      return { type: ProgramInstruction.Initialize, args: argsCodec.decode(data.slice(INITIALIZE_INSTRUCTION_DISCRIMINATOR.length)) };
    }
    if (matchDisc(data, REGISTER_AGENT_INSTRUCTION_DISCRIMINATOR)) {
      const argsCodec = getStructCodec([
        ["name", PodStringCodec],
      ]);
      return { type: ProgramInstruction.RegisterAgent, args: argsCodec.decode(data.slice(REGISTER_AGENT_INSTRUCTION_DISCRIMINATOR.length)) };
    }
    if (matchDisc(data, DELETE_AGENT_INSTRUCTION_DISCRIMINATOR)) {
      const argsCodec = getStructCodec([
        ["id", getAddressCodec()],
      ]);
      return { type: ProgramInstruction.DeleteAgent, args: argsCodec.decode(data.slice(DELETE_AGENT_INSTRUCTION_DISCRIMINATOR.length)) };
    }
    if (matchDisc(data, DEPOSIT_FOR_AGENT_USE_INSTRUCTION_DISCRIMINATOR)) {
      const argsCodec = getStructCodec([
        ["amount", getU64Codec()],
      ]);
      return { type: ProgramInstruction.DepositForAgentUse, args: argsCodec.decode(data.slice(DEPOSIT_FOR_AGENT_USE_INSTRUCTION_DISCRIMINATOR.length)) };
    }
    if (matchDisc(data, REGISTER_TICKER_FOR_ME_INSTRUCTION_DISCRIMINATOR)) {
      const argsCodec = getStructCodec([
        ["amountToSpend", getU64Codec()],
      ]);
      return { type: ProgramInstruction.RegisterTickerForMe, args: argsCodec.decode(data.slice(REGISTER_TICKER_FOR_ME_INSTRUCTION_DISCRIMINATOR.length)) };
    }
    return null;
  }

  createInitializeInstruction(input: InitializeInstructionInput): IInstruction {
    const argsCodec = getStructCodec([
      ["args", InitializeArgsCodec],
    ]);
    const data = Uint8Array.from([0, ...argsCodec.encode({ args: input.args })]);
    return {
      programAddress: PROGRAM_ADDRESS,
      accounts: [
        { address: input.payer, role: AccountRole.WRITABLE_SIGNER },
        { address: input.globalStateAccount, role: AccountRole.READONLY },
        { address: input.destinationTokenAccount, role: AccountRole.WRITABLE },
        { address: input.systemProgram, role: AccountRole.READONLY },
      ],
      data,
    };
  }

  createRegisterAgentInstruction(input: RegisterAgentInstructionInput): IInstruction {
    const argsCodec = getStructCodec([
      ["name", PodStringCodec],
    ]);
    const data = Uint8Array.from([1, ...argsCodec.encode({ name: input.name })]);
    return {
      programAddress: PROGRAM_ADDRESS,
      accounts: [
        { address: input.admin, role: AccountRole.WRITABLE_SIGNER },
        { address: input.agent, role: AccountRole.WRITABLE },
        { address: input.globalStateAccount, role: AccountRole.READONLY },
        { address: input.systemProgram, role: AccountRole.READONLY },
      ],
      data,
    };
  }

  createDeleteAgentInstruction(input: DeleteAgentInstructionInput): IInstruction {
    const argsCodec = getStructCodec([
      ["id", getAddressCodec()],
    ]);
    const data = Uint8Array.from([2, ...argsCodec.encode({ id: input.id })]);
    return {
      programAddress: PROGRAM_ADDRESS,
      accounts: [
        { address: input.admin, role: AccountRole.WRITABLE_SIGNER },
        { address: input.agent, role: AccountRole.WRITABLE },
        { address: input.globalStateAccount, role: AccountRole.READONLY },
        { address: input.systemProgram, role: AccountRole.READONLY },
      ],
      data,
    };
  }

  createDepositForAgentUseInstruction(input: DepositForAgentUseInstructionInput): IInstruction {
    const argsCodec = getStructCodec([
      ["amount", getU64Codec()],
    ]);
    const data = Uint8Array.from([3, ...argsCodec.encode({ amount: input.amount })]);
    return {
      programAddress: PROGRAM_ADDRESS,
      accounts: [
        { address: input.payer, role: AccountRole.WRITABLE_SIGNER },
        { address: input.user, role: AccountRole.WRITABLE_SIGNER },
        { address: input.agent, role: AccountRole.WRITABLE },
        { address: input.globalStateAccount, role: AccountRole.WRITABLE },
        { address: input.userState, role: AccountRole.READONLY },
        { address: input.userStateVault, role: AccountRole.READONLY },
        { address: input.ticker, role: AccountRole.READONLY },
        { address: input.destinationFeeTokenAccount, role: AccountRole.WRITABLE },
        { address: input.userTokenAccount, role: AccountRole.WRITABLE },
        { address: input.mint, role: AccountRole.READONLY },
        { address: input.tokenProgram, role: AccountRole.READONLY },
        { address: input.systemProgram, role: AccountRole.READONLY },
      ],
      data,
    };
  }

  createRegisterTickerForMeInstruction(input: RegisterTickerForMeInstructionInput): IInstruction {
    const argsCodec = getStructCodec([
      ["amountToSpend", getU64Codec()],
    ]);
    const data = Uint8Array.from([4, ...argsCodec.encode({ amountToSpend: input.amountToSpend })]);
    return {
      programAddress: PROGRAM_ADDRESS,
      accounts: [
        { address: input.payer, role: AccountRole.WRITABLE_SIGNER },
        { address: input.user, role: AccountRole.WRITABLE_SIGNER },
        { address: input.agent, role: AccountRole.WRITABLE },
        { address: input.userState, role: AccountRole.READONLY },
        { address: input.userStateVault, role: AccountRole.READONLY },
        { address: input.ticker, role: AccountRole.READONLY },
        { address: input.mint, role: AccountRole.READONLY },
        { address: input.tokenProgram, role: AccountRole.READONLY },
        { address: input.systemProgram, role: AccountRole.READONLY },
      ],
      data,
    };
  }
}

/* Errors */
export const PROGRAM_ERRORS: Record<number, { name: string; msg?: string }> = {
  0: { name: "Unauthorized" },
};


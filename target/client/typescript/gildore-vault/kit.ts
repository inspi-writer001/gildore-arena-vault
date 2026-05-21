import { type Address, address, AccountRole, type Instruction, getProgramDerivedAddress, getAddressCodec } from "@solana/kit";
import { addCodecSizePrefix, fixCodecSize, getArrayCodec, getBooleanCodec, getBytesCodec, getStructCodec, getU16Codec, getU64Codec, getU8Codec, getUtf8Codec } from "@solana/codecs";

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
  seeds: Uint8Array;
}

export interface Ticker {
  amountToSpend: bigint;
}

export interface InitializeArgs {
  feeBps: number;
  maxFee: bigint;
  admin: Array<Address>;
}

export interface InitializeInstructionArgs {
  args: InitializeArgs;
}

export interface RegisterAgentInstructionArgs {
  name: string;
}

export interface DeleteAgentInstructionArgs {
  name: string;
}

export interface DepositForAgentUseInstructionArgs {
  amount: bigint;
}

export interface RegisterTickerForMeInstructionArgs {
  amountToSpend: bigint;
}

export interface InitializeInstructionInput {
  payer: Address;
  destinationTokenAccount: Address;
  systemProgram: Address;
  args: InitializeArgs;
}

export interface RegisterAgentInstructionInput {
  admin: Address;
  agent: Address;
  systemProgram: Address;
  name: string;
}

export interface DeleteAgentInstructionInput {
  admin: Address;
  agent: Address;
  systemProgram: Address;
  name: string;
}

export interface DepositForAgentUseInstructionInput {
  payer: Address;
  user: Address;
  userStateVault: Address;
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
  userStateVault: Address;
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

export const GlobalStateCodec = {
  encode(value: GlobalState): Uint8Array {
    const fixedCodec = getStructCodec([
      ["feeDestination", getAddressCodec()],
      ["feeBps", getU16Codec()],
      ["maxFee", getU64Codec()],
      ["bump", getU8Codec()],
    ]);
    const fixedBytes = fixedCodec.encode({ feeDestination: value.feeDestination, feeBps: value.feeBps, maxFee: value.maxFee, bump: value.bump });
    const adminPrefix = getU16Codec().encode(value.admin.length);
    const adminBytes = getArrayCodec(getAddressCodec(), { size: value.admin.length }).encode(value.admin);
    return Uint8Array.from([...fixedBytes, ...adminPrefix, ...adminBytes]);
  },
  decode(data: Uint8Array): GlobalState {
    let offset = 0;
    const fixedCodec = getStructCodec([
      ["feeDestination", getAddressCodec()],
      ["feeBps", getU16Codec()],
      ["maxFee", getU64Codec()],
      ["bump", getU8Codec()],
    ]);
    const fixedResult = fixedCodec.decode(data.slice(offset));
    offset += fixedCodec.fixedSize ?? fixedCodec.encode(fixedResult).length;
    const adminLen = getU16Codec().decode(data.slice(offset));
    offset += 2;
    const adminCodec = getArrayCodec(getAddressCodec(), { size: Number(adminLen) });
    const admin = adminCodec.decode(data.slice(offset));
    offset += adminCodec.encode(admin).length;
    return { feeDestination: fixedResult.feeDestination, feeBps: fixedResult.feeBps, maxFee: fixedResult.maxFee, bump: fixedResult.bump, admin };
  },
};

export const AgentMarketCodec = getStructCodec([
  ["agentId", getAddressCodec()],
  ["tickerId", getAddressCodec()],
  ["isTrading", getBooleanCodec()],
  ["bump", getU8Codec()],
]);

export const IAgentCodec = getStructCodec([
  ["agentId", getAddressCodec()],
  ["bump", getU8Codec()],
  ["seeds", fixCodecSize(getBytesCodec(), 37)],
]);

export const TickerCodec = getStructCodec([
  ["amountToSpend", getU64Codec()],
]);

export const InitializeArgsCodec = {
  encode(value: InitializeArgs): Uint8Array {
    const fixedCodec = getStructCodec([
      ["feeBps", getU16Codec()],
      ["maxFee", getU64Codec()],
    ]);
    const fixedBytes = fixedCodec.encode({ feeBps: value.feeBps, maxFee: value.maxFee });
    const adminPrefix = getU16Codec().encode(value.admin.length);
    const adminBytes = getArrayCodec(getAddressCodec(), { size: value.admin.length }).encode(value.admin);
    return Uint8Array.from([...fixedBytes, ...adminPrefix, ...adminBytes]);
  },
  decode(data: Uint8Array): InitializeArgs {
    let offset = 0;
    const fixedCodec = getStructCodec([
      ["feeBps", getU16Codec()],
      ["maxFee", getU64Codec()],
    ]);
    const fixedResult = fixedCodec.decode(data.slice(offset));
    offset += fixedCodec.fixedSize ?? fixedCodec.encode(fixedResult).length;
    const adminLen = getU16Codec().decode(data.slice(offset));
    offset += 2;
    const adminCodec = getArrayCodec(getAddressCodec(), { size: Number(adminLen) });
    const admin = adminCodec.decode(data.slice(offset));
    offset += adminCodec.encode(admin).length;
    return { feeBps: fixedResult.feeBps, maxFee: fixedResult.maxFee, admin };
  },
};

/* Enums */
export const ProgramInstruction = {
  Initialize: "Initialize",
  RegisterAgent: "RegisterAgent",
  DeleteAgent: "DeleteAgent",
  DepositForAgentUse: "DepositForAgentUse",
  RegisterTickerForMe: "RegisterTickerForMe",
} as const;

export type ProgramInstruction =
  (typeof ProgramInstruction)[keyof typeof ProgramInstruction];

export type DecodedInstruction =
  | { type: typeof ProgramInstruction.Initialize; args: InitializeInstructionArgs }
  | { type: typeof ProgramInstruction.RegisterAgent; args: RegisterAgentInstructionArgs }
  | { type: typeof ProgramInstruction.DeleteAgent; args: DeleteAgentInstructionArgs }
  | { type: typeof ProgramInstruction.DepositForAgentUse; args: DepositForAgentUseInstructionArgs }
  | { type: typeof ProgramInstruction.RegisterTickerForMe; args: RegisterTickerForMeInstructionArgs };

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
      let offset = REGISTER_AGENT_INSTRUCTION_DISCRIMINATOR.length;
      const nameLen = getU8Codec().decode(data.slice(offset));
      offset += 1;
      const name = new TextDecoder().decode(data.slice(offset, offset + Number(nameLen)));
      offset += Number(nameLen);
      return { type: ProgramInstruction.RegisterAgent, args: { name } };
    }
    if (matchDisc(data, DELETE_AGENT_INSTRUCTION_DISCRIMINATOR)) {
      let offset = DELETE_AGENT_INSTRUCTION_DISCRIMINATOR.length;
      const nameLen = getU8Codec().decode(data.slice(offset));
      offset += 1;
      const name = new TextDecoder().decode(data.slice(offset, offset + Number(nameLen)));
      offset += Number(nameLen);
      return { type: ProgramInstruction.DeleteAgent, args: { name } };
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

  async createInitializeInstruction(input: InitializeInstructionInput): Promise<Instruction> {
    const accountsMap: Record<string, Address> = {};
    accountsMap["globalStateAccount"] = await findGlobalStateAccountAddress();
    const argsCodec = getStructCodec([
      ["args", InitializeArgsCodec],
    ]);
    const data = Uint8Array.from([0, ...argsCodec.encode({ args: input.args })]);
    return {
      programAddress: PROGRAM_ADDRESS,
      accounts: [
        { address: input.payer, role: AccountRole.WRITABLE_SIGNER },
        { address: accountsMap["globalStateAccount"], role: AccountRole.WRITABLE },
        { address: input.destinationTokenAccount, role: AccountRole.WRITABLE },
        { address: input.systemProgram, role: AccountRole.READONLY },
      ],
      data,
    };
  }

  async createRegisterAgentInstruction(input: RegisterAgentInstructionInput): Promise<Instruction> {
    const accountsMap: Record<string, Address> = {};
    accountsMap["globalStateAccount"] = await findGlobalStateAccountAddress();
    const disc = new Uint8Array([1]);
    const fixedBytes = new Uint8Array(0);
    const nameBytes = new TextEncoder().encode(input.name);
    const namePrefix = getU8Codec().encode(nameBytes.length);
    const data = Uint8Array.from([...disc, ...fixedBytes, ...namePrefix, ...nameBytes]);
    return {
      programAddress: PROGRAM_ADDRESS,
      accounts: [
        { address: input.admin, role: AccountRole.WRITABLE_SIGNER },
        { address: input.agent, role: AccountRole.WRITABLE },
        { address: accountsMap["globalStateAccount"], role: AccountRole.READONLY },
        { address: input.systemProgram, role: AccountRole.READONLY },
      ],
      data,
    };
  }

  async createDeleteAgentInstruction(input: DeleteAgentInstructionInput): Promise<Instruction> {
    const accountsMap: Record<string, Address> = {};
    accountsMap["globalStateAccount"] = await findGlobalStateAccountAddress();
    const disc = new Uint8Array([2]);
    const fixedBytes = new Uint8Array(0);
    const nameBytes = new TextEncoder().encode(input.name);
    const namePrefix = getU8Codec().encode(nameBytes.length);
    const data = Uint8Array.from([...disc, ...fixedBytes, ...namePrefix, ...nameBytes]);
    return {
      programAddress: PROGRAM_ADDRESS,
      accounts: [
        { address: input.admin, role: AccountRole.WRITABLE_SIGNER },
        { address: input.agent, role: AccountRole.WRITABLE },
        { address: accountsMap["globalStateAccount"], role: AccountRole.READONLY },
        { address: input.systemProgram, role: AccountRole.READONLY },
      ],
      data,
    };
  }

  async createDepositForAgentUseInstruction(input: DepositForAgentUseInstructionInput): Promise<Instruction> {
    const accountsMap: Record<string, Address> = {};
    accountsMap["agent"] = await findAgentAddress();
    accountsMap["globalStateAccount"] = await findGlobalStateAccountAddress();
    accountsMap["userState"] = await findUserStateAddress();
    accountsMap["ticker"] = await findTickerAddress();
    const argsCodec = getStructCodec([
      ["amount", getU64Codec()],
    ]);
    const data = Uint8Array.from([3, ...argsCodec.encode({ amount: input.amount })]);
    return {
      programAddress: PROGRAM_ADDRESS,
      accounts: [
        { address: input.payer, role: AccountRole.WRITABLE_SIGNER },
        { address: input.user, role: AccountRole.WRITABLE_SIGNER },
        { address: accountsMap["agent"], role: AccountRole.WRITABLE },
        { address: accountsMap["globalStateAccount"], role: AccountRole.WRITABLE },
        { address: accountsMap["userState"], role: AccountRole.READONLY },
        { address: input.userStateVault, role: AccountRole.READONLY },
        { address: accountsMap["ticker"], role: AccountRole.READONLY },
        { address: input.destinationFeeTokenAccount, role: AccountRole.WRITABLE },
        { address: input.userTokenAccount, role: AccountRole.WRITABLE },
        { address: input.mint, role: AccountRole.READONLY },
        { address: input.tokenProgram, role: AccountRole.READONLY },
        { address: input.systemProgram, role: AccountRole.READONLY },
      ],
      data,
    };
  }

  async createRegisterTickerForMeInstruction(input: RegisterTickerForMeInstructionInput): Promise<Instruction> {
    const accountsMap: Record<string, Address> = {};
    accountsMap["agent"] = await findAgentAddress();
    accountsMap["userState"] = await findUserStateAddress();
    accountsMap["ticker"] = await findTickerAddress();
    const argsCodec = getStructCodec([
      ["amountToSpend", getU64Codec()],
    ]);
    const data = Uint8Array.from([4, ...argsCodec.encode({ amountToSpend: input.amountToSpend })]);
    return {
      programAddress: PROGRAM_ADDRESS,
      accounts: [
        { address: input.payer, role: AccountRole.WRITABLE_SIGNER },
        { address: input.user, role: AccountRole.WRITABLE_SIGNER },
        { address: accountsMap["agent"], role: AccountRole.WRITABLE },
        { address: accountsMap["userState"], role: AccountRole.READONLY },
        { address: input.userStateVault, role: AccountRole.READONLY },
        { address: accountsMap["ticker"], role: AccountRole.READONLY },
        { address: input.mint, role: AccountRole.READONLY },
        { address: input.tokenProgram, role: AccountRole.READONLY },
        { address: input.systemProgram, role: AccountRole.READONLY },
      ],
      data,
    };
  }
}

/* PDA Helpers */
export async function findGlobalStateAccountAddress(): Promise<Address> {
  return (await getProgramDerivedAddress({
    programAddress: PROGRAM_ADDRESS,
    seeds: [
        new Uint8Array([103, 108, 111, 98, 97, 108, 95, 115, 116, 97, 116, 101]),
    ],
  }))[0];
}

export async function findAgentAddress(): Promise<Address> {
  return (await getProgramDerivedAddress({
    programAddress: PROGRAM_ADDRESS,
    seeds: [
        new Uint8Array([97, 103, 101, 110, 116]),
    ],
  }))[0];
}

export async function findUserStateAddress(): Promise<Address> {
  return (await getProgramDerivedAddress({
    programAddress: PROGRAM_ADDRESS,
    seeds: [
        new Uint8Array([117, 115, 101, 114, 95, 115, 116, 97, 116, 101]),
    ],
  }))[0];
}

export async function findTickerAddress(): Promise<Address> {
  return (await getProgramDerivedAddress({
    programAddress: PROGRAM_ADDRESS,
    seeds: [
        new Uint8Array([116, 105, 99, 107, 101, 114]),
    ],
  }))[0];
}

/* Errors */
export const PROGRAM_ERRORS: Record<number, { name: string; msg?: string }> = {
  0: { name: "Unauthorized" },
};


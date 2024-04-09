/* eslint-disable */
import * as _m0 from "protobufjs/minimal";
import Long = require("long");

export const protobufPackage = "";

export interface AddEventRequest {
  a: number;
  b: number;
}

export interface AddEventResponse {
  result: number;
}

function createBaseAddEventRequest(): AddEventRequest {
  return { a: 0, b: 0 };
}

export const AddEventRequest = {
  encode(message: AddEventRequest, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.a !== 0) {
      writer.uint32(8).int64(message.a);
    }
    if (message.b !== 0) {
      writer.uint32(16).int64(message.b);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): AddEventRequest {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseAddEventRequest();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 8) {
            break;
          }

          message.a = longToNumber(reader.int64() as Long);
          continue;
        case 2:
          if (tag !== 16) {
            break;
          }

          message.b = longToNumber(reader.int64() as Long);
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): AddEventRequest {
    return {
      a: isSet(object.a) ? globalThis.Number(object.a) : 0,
      b: isSet(object.b) ? globalThis.Number(object.b) : 0,
    };
  },

  toJSON(message: AddEventRequest): unknown {
    const obj: any = {};
    if (message.a !== 0) {
      obj.a = Math.round(message.a);
    }
    if (message.b !== 0) {
      obj.b = Math.round(message.b);
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<AddEventRequest>, I>>(base?: I): AddEventRequest {
    return AddEventRequest.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<AddEventRequest>, I>>(object: I): AddEventRequest {
    const message = createBaseAddEventRequest();
    message.a = object.a ?? 0;
    message.b = object.b ?? 0;
    return message;
  },
};

function createBaseAddEventResponse(): AddEventResponse {
  return { result: 0 };
}

export const AddEventResponse = {
  encode(message: AddEventResponse, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.result !== 0) {
      writer.uint32(8).int64(message.result);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): AddEventResponse {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseAddEventResponse();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 8) {
            break;
          }

          message.result = longToNumber(reader.int64() as Long);
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): AddEventResponse {
    return { result: isSet(object.result) ? globalThis.Number(object.result) : 0 };
  },

  toJSON(message: AddEventResponse): unknown {
    const obj: any = {};
    if (message.result !== 0) {
      obj.result = Math.round(message.result);
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<AddEventResponse>, I>>(base?: I): AddEventResponse {
    return AddEventResponse.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<AddEventResponse>, I>>(object: I): AddEventResponse {
    const message = createBaseAddEventResponse();
    message.result = object.result ?? 0;
    return message;
  },
};

type Builtin = Date | Function | Uint8Array | string | number | boolean | undefined;

export type DeepPartial<T> = T extends Builtin ? T
  : T extends globalThis.Array<infer U> ? globalThis.Array<DeepPartial<U>>
  : T extends ReadonlyArray<infer U> ? ReadonlyArray<DeepPartial<U>>
  : T extends {} ? { [K in keyof T]?: DeepPartial<T[K]> }
  : Partial<T>;

type KeysOfUnion<T> = T extends T ? keyof T : never;
export type Exact<P, I extends P> = P extends Builtin ? P
  : P & { [K in keyof P]: Exact<P[K], I[K]> } & { [K in Exclude<keyof I, KeysOfUnion<P>>]: never };

function longToNumber(long: Long): number {
  if (long.gt(globalThis.Number.MAX_SAFE_INTEGER)) {
    throw new globalThis.Error("Value is larger than Number.MAX_SAFE_INTEGER");
  }
  return long.toNumber();
}

if (_m0.util.Long !== Long) {
  _m0.util.Long = Long as any;
  _m0.configure();
}

function isSet(value: any): boolean {
  return value !== null && value !== undefined;
}

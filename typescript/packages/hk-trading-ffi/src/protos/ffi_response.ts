/* eslint-disable */
import * as _m0 from "protobufjs/minimal";
import { HkFFIEventType, hkFFIEventTypeFromJSON, hkFFIEventTypeToJSON } from "./ffi_event_type";

export const protobufPackage = "";

export enum HkFFIStatusCode {
  Ok = 0,
  Err = 1,
  Internal = 2,
  UNRECOGNIZED = -1,
}

export function hkFFIStatusCodeFromJSON(object: any): HkFFIStatusCode {
  switch (object) {
    case 0:
    case "Ok":
      return HkFFIStatusCode.Ok;
    case 1:
    case "Err":
      return HkFFIStatusCode.Err;
    case 2:
    case "Internal":
      return HkFFIStatusCode.Internal;
    case -1:
    case "UNRECOGNIZED":
    default:
      return HkFFIStatusCode.UNRECOGNIZED;
  }
}

export function hkFFIStatusCodeToJSON(object: HkFFIStatusCode): string {
  switch (object) {
    case HkFFIStatusCode.Ok:
      return "Ok";
    case HkFFIStatusCode.Err:
      return "Err";
    case HkFFIStatusCode.Internal:
      return "Internal";
    case HkFFIStatusCode.UNRECOGNIZED:
    default:
      return "UNRECOGNIZED";
  }
}

export interface HkFFIResponse {
  event: HkFFIEventType;
  payload: Uint8Array;
  code: HkFFIStatusCode;
}

function createBaseHkFFIResponse(): HkFFIResponse {
  return { event: 0, payload: new Uint8Array(0), code: 0 };
}

export const HkFFIResponse = {
  encode(message: HkFFIResponse, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.event !== 0) {
      writer.uint32(8).int32(message.event);
    }
    if (message.payload.length !== 0) {
      writer.uint32(18).bytes(message.payload);
    }
    if (message.code !== 0) {
      writer.uint32(24).int32(message.code);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): HkFFIResponse {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseHkFFIResponse();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 8) {
            break;
          }

          message.event = reader.int32() as any;
          continue;
        case 2:
          if (tag !== 18) {
            break;
          }

          message.payload = reader.bytes();
          continue;
        case 3:
          if (tag !== 24) {
            break;
          }

          message.code = reader.int32() as any;
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): HkFFIResponse {
    return {
      event: isSet(object.event) ? hkFFIEventTypeFromJSON(object.event) : 0,
      payload: isSet(object.payload) ? bytesFromBase64(object.payload) : new Uint8Array(0),
      code: isSet(object.code) ? hkFFIStatusCodeFromJSON(object.code) : 0,
    };
  },

  toJSON(message: HkFFIResponse): unknown {
    const obj: any = {};
    if (message.event !== 0) {
      obj.event = hkFFIEventTypeToJSON(message.event);
    }
    if (message.payload.length !== 0) {
      obj.payload = base64FromBytes(message.payload);
    }
    if (message.code !== 0) {
      obj.code = hkFFIStatusCodeToJSON(message.code);
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<HkFFIResponse>, I>>(base?: I): HkFFIResponse {
    return HkFFIResponse.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<HkFFIResponse>, I>>(object: I): HkFFIResponse {
    const message = createBaseHkFFIResponse();
    message.event = object.event ?? 0;
    message.payload = object.payload ?? new Uint8Array(0);
    message.code = object.code ?? 0;
    return message;
  },
};

function bytesFromBase64(b64: string): Uint8Array {
  if ((globalThis as any).Buffer) {
    return Uint8Array.from(globalThis.Buffer.from(b64, "base64"));
  } else {
    const bin = globalThis.atob(b64);
    const arr = new Uint8Array(bin.length);
    for (let i = 0; i < bin.length; ++i) {
      arr[i] = bin.charCodeAt(i);
    }
    return arr;
  }
}

function base64FromBytes(arr: Uint8Array): string {
  if ((globalThis as any).Buffer) {
    return globalThis.Buffer.from(arr).toString("base64");
  } else {
    const bin: string[] = [];
    arr.forEach((byte) => {
      bin.push(globalThis.String.fromCharCode(byte));
    });
    return globalThis.btoa(bin.join(""));
  }
}

type Builtin = Date | Function | Uint8Array | string | number | boolean | undefined;

export type DeepPartial<T> = T extends Builtin ? T
  : T extends globalThis.Array<infer U> ? globalThis.Array<DeepPartial<U>>
  : T extends ReadonlyArray<infer U> ? ReadonlyArray<DeepPartial<U>>
  : T extends {} ? { [K in keyof T]?: DeepPartial<T[K]> }
  : Partial<T>;

type KeysOfUnion<T> = T extends T ? keyof T : never;
export type Exact<P, I extends P> = P extends Builtin ? P
  : P & { [K in keyof P]: Exact<P[K], I[K]> } & { [K in Exclude<keyof I, KeysOfUnion<P>>]: never };

function isSet(value: any): boolean {
  return value !== null && value !== undefined;
}

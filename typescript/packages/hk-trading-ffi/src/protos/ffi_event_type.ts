/* eslint-disable */

export const protobufPackage = "";

export enum HkFFIEventType {
  HK_FFI_REQ_UNKNOWN = 0,
  /** HK_FFI_REQ_CHART_GET_SVG - Chart */
  HK_FFI_REQ_CHART_GET_SVG = 1000,
  /** HK_FFI_RES_CHART_GET_SVG - HK_FFI_REQ_CHART_ */
  HK_FFI_RES_CHART_GET_SVG = 1001,
  /** HK_FFI_REQ_SYMBOL_GET_INFO - Symbol */
  HK_FFI_REQ_SYMBOL_GET_INFO = 2000,
  UNRECOGNIZED = -1,
}

export function hkFFIEventTypeFromJSON(object: any): HkFFIEventType {
  switch (object) {
    case 0:
    case "HK_FFI_REQ_UNKNOWN":
      return HkFFIEventType.HK_FFI_REQ_UNKNOWN;
    case 1000:
    case "HK_FFI_REQ_CHART_GET_SVG":
      return HkFFIEventType.HK_FFI_REQ_CHART_GET_SVG;
    case 1001:
    case "HK_FFI_RES_CHART_GET_SVG":
      return HkFFIEventType.HK_FFI_RES_CHART_GET_SVG;
    case 2000:
    case "HK_FFI_REQ_SYMBOL_GET_INFO":
      return HkFFIEventType.HK_FFI_REQ_SYMBOL_GET_INFO;
    case -1:
    case "UNRECOGNIZED":
    default:
      return HkFFIEventType.UNRECOGNIZED;
  }
}

export function hkFFIEventTypeToJSON(object: HkFFIEventType): string {
  switch (object) {
    case HkFFIEventType.HK_FFI_REQ_UNKNOWN:
      return "HK_FFI_REQ_UNKNOWN";
    case HkFFIEventType.HK_FFI_REQ_CHART_GET_SVG:
      return "HK_FFI_REQ_CHART_GET_SVG";
    case HkFFIEventType.HK_FFI_RES_CHART_GET_SVG:
      return "HK_FFI_RES_CHART_GET_SVG";
    case HkFFIEventType.HK_FFI_REQ_SYMBOL_GET_INFO:
      return "HK_FFI_REQ_SYMBOL_GET_INFO";
    case HkFFIEventType.UNRECOGNIZED:
    default:
      return "UNRECOGNIZED";
  }
}

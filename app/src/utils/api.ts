import { invoke } from "@tauri-apps/api";
import { Request, Response } from "@macrograph/core-types";

export type RequestType = Request["type"];
export type RequestData<
  T extends RequestType,
  R = Extract<Request, { type: T }>
> = R extends { data: any } ? R["data"] : never;
export type ResponseData<
  T extends RequestType,
  R = Extract<Response, { type: T }>
> = R extends { data: any } ? R["data"] : never;

export const send = async <T extends RequestType>(
  type: T,
  data?: RequestData<T>
): Promise<ResponseData<T>> => {
  const res: any = await invoke("core_request", {
    req: {
      type,
      data,
    },
  });

  return res.data;
};

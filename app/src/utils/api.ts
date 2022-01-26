import { invoke } from "@tauri-apps/api";
import { Request, Response } from "@macrograph/core-types";

export const send = async <
  T extends Request["type"],
  R = Extract<Response, { type: T }>,
  // @ts-expect-error
  D = R["data"] extends object ? R["data"] : never
>(
  type: T,
  // @ts-expect-error
  data?: Extract<Request, { type: T }>["data"]
): Promise<D> => {
  const res: any = await invoke("core_request", {
    req: {
      type,
      data,
    },
  });

  return res.data;
};

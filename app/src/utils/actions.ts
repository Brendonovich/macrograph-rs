import { runInAction } from "mobx";
import { RequestData, RequestType, ResponseData, send } from ".";

interface ActionArgs<T extends RequestType> {
  request: T;
  data?: RequestData<T>;
  run: (data: ResponseData<T>) => void;
}

export const action = async <T extends RequestType>(args: ActionArgs<T>) => {
  const data = await send(args.request, args.data);

  runInAction(() => args.run(data));
};

const debounceCache = new Map<string, NodeJS.Timeout>();

interface DebounceActionArgs<T extends RequestType> {
  request: T;
  data?: RequestData<T>;
  timeout: number;
  key: string;
}

export const debouncedAction = async <T extends RequestType>(
  args: DebounceActionArgs<T>
) => {
  let key = `${args.request}-${args.key}`;
  let prevTimeout = debounceCache.get(key);
  if (prevTimeout !== undefined) {
    clearTimeout(prevTimeout);
  }

  debounceCache.set(
    key,
    setTimeout(() => {
      send(args.request, args.data);
      debounceCache.delete(key);
    }, args.timeout)
  );
};

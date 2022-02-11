import clsx from "clsx";
import { makeAutoObservable } from "mobx";
import { Observer } from "mobx-react-lite";
import { FC, useState } from "react";
import { NodeSchemaType } from "@macrograph/core-types";

import { useCore } from "~/contexts";
import { NodeSchema } from "~/models";
import { Package } from "~/models/Package";

class SchemaMenuState {
  openPackages: Set<Package> = new Set();

  constructor() {
    makeAutoObservable(this);
  }

  togglePackage(pkg: Package) {
    if (this.openPackages.has(pkg)) this.openPackages.delete(pkg);
    else this.openPackages.add(pkg);
  }
}

interface Props {
  onSchemaClicked(s: NodeSchema): void | Promise<void>;
  position: XY;
}

const TypeIndicatorColours: Record<NodeSchemaType, string> = {
  Base: "bg-gray-base",
  Exec: "bg-blue-exec",
  Event: "bg-red-event",
  Pure: "bg-green-pure",
};

export const SchemaMenu: FC<Props> = ({ onSchemaClicked, position }) => {
  const core = useCore();

  const [state] = useState(new SchemaMenuState());

  const [search, setSearch] = useState("");
  const lowercaseSearchTokens = search
    .toLowerCase()
    .split(" ")
    .filter((s) => s !== "");

  return (
    <Observer>
      {() => (
        <div
          className="flex flex-col bg-neutral-900 border-white text-white border absolute z-10 w-80 h-[30rem] rounded-md shadow-md overflow-hidden text-sm"
          style={{
            left: position.x - 20,
            top: position.y - 20,
          }}
        >
          <div className="p-2">
            <input
              onChange={(e) => setSearch(e.target.value)}
              className="text-black w-full px-2 py-0.5 rounded"
              autoFocus
              placeholder="Search Nodes..."
              autoComplete="false"
              autoCapitalize="false"
              autoCorrect="false"
              spellCheck="false"
            />
          </div>
          <div className="p-2 pt-0 flex-1 overflow-auto">
            <div className="">
              {core.packages.map((p) => {
                let open = state.openPackages.has(p) || search !== "";

                const lowercasePackageName = p.name.toLowerCase();

                const leftoverSearchTokens = lowercaseSearchTokens.filter(
                  (s) => !lowercasePackageName.includes(s)
                );

                let filteredSchemas = p.schemas.filter((s) => {
                  let lowercaseSchemaName = s.name.toLowerCase();

                  return leftoverSearchTokens.every((t) =>
                    lowercaseSchemaName.includes(t)
                  );
                });

                if (filteredSchemas.length === 0) return null;

                return (
                  <div key={p.name}>
                    <button
                      className="px-2 py-0.5 flex flex-row items-center space-x-2 hover:bg-neutral-700 min-w-full text-left rounded-md"
                      onClick={() => state.togglePackage(p)}
                    >
                      <div className="w-2">{open ? "v" : ">"}</div>
                      <span>{p.name}</span>
                    </button>
                    {open && (
                      <div className="pl-4">
                        {filteredSchemas.map((s) => (
                          <div key={s.name}>
                            <button
                              className="px-2 py-0.5 flex flex-row items-center space-x-2 whitespace-nowrap min-w-full text-left hover:bg-neutral-700 rounded-lg"
                              onClick={() => onSchemaClicked(s)}
                            >
                              <div
                                className={clsx(
                                  "h-3 w-3 rounded-full",
                                  TypeIndicatorColours[s.type]
                                )}
                              />
                              <span>{s.name}</span>
                            </button>
                          </div>
                        ))}
                      </div>
                    )}
                  </div>
                );
              })}
            </div>
          </div>
        </div>
      )}
    </Observer>
  );
};

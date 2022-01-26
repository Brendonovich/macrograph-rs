import { makeAutoObservable } from "mobx";
import { Observer } from "mobx-react-lite";
import { FC, useState } from "react";
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

export const SchemaMenu: FC<Props> = ({ onSchemaClicked, position }) => {
  const core = useCore();

  const [state] = useState(new SchemaMenuState());

  return (
    <Observer>
      {() => (
        <div
          className="bg-neutral-900 border-white text-white border absolute z-10 w-72 h-[30rem] rounded-md shadow-md"
          style={{
            left: position.x,
            top: position.y,
          }}
        >
          <div className="p-2 overflow-auto h-full space-y-1">
            {core.packages.map((p) => {
              let open = state.openPackages.has(p);

              return (
                <div key={p.name}>
                  <button
                    className="px-2 hover:bg-neutral-700 min-w-full text-left rounded-lg"
                    onClick={() => state.togglePackage(p)}
                  >
                    {p.name}
                  </button>
                  {open && (
                    <div className="pl-6 space-y-1">
                      {p.schemas.map((s) => (
                        <div key={s.name} className="first:mt-1">
                          <button
                            className="px-2 whitespace-nowrap min-w-full text-left hover:bg-neutral-700 rounded-lg"
                            onClick={() => onSchemaClicked(s)}
                          >
                            {s.name}
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
      )}
    </Observer>
  );
};

import { makeAutoObservable } from "mobx";
import { runInAction } from "mobx";
import { send } from "~/utils";

import { Graph } from "./Graph";
import { Package } from "./Package";

export class Core {
  graph: Graph = new Graph();
  packages: Package[] = [];

  constructor() {
    makeAutoObservable(this);
  }

  async loadPackages() {
    const res = await send("GetPackages");

    runInAction(() => {
      this.packages = res.packages.map((pkg) => new Package(pkg));
    });
  }

  schema(pkg: string, schema: string) {
    return this.packages.find((p) => p.name === pkg)?.schema(schema);
  }
}

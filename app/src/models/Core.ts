import { makeAutoObservable } from "mobx";
import { runInAction } from "mobx";
import { send } from "~/utils";

import { Graph } from "./Graph";
import { Package } from "./Package";

export class Core {
  graphs: Record<number, Graph> = {};
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

  async loadProject() {
    const res = await send("GetProject");

    runInAction(() => {
      this.graphs = res.graphs.reduce(
        (acc, g) => ({
          ...acc,
          [g.id]: new Graph({ ...g, core: this }),
        }),
        {}
      );
    });
  }

  async createGraph() {
    const graph = await send("CreateGraph");
    return runInAction(() => {
      return this.graphs[graph.id] = new Graph({
        core: this,
        nodes: [],
        ...graph,
      })
    });
  }

  schema(pkg: string, name: string) {
    return this.packages.find((p) => p.name === pkg)?.schema(name);
  }
}

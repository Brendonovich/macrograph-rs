import { Package as PackageRaw } from "@macrograph/core-types";
import { makeAutoObservable } from "mobx";

import { NodeSchema } from "./NodeSchema";

export class Package {
  name: string;
  schemas: NodeSchema[];

  constructor(pkg: PackageRaw) {
    this.name = pkg.name;
    this.schemas = pkg.schemas.map((s) => new NodeSchema(s, this));
    makeAutoObservable(this);
  }

  schema(name: string) {
    return this.schemas.find((s) => s.name === name);
  }
}

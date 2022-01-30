import { makeAutoObservable } from "mobx";
import {
  NodeSchema as RawNodeSchema,
  NodeSchemaType,
} from "@macrograph/core-types";
import { Package } from "./Package";

export class NodeSchema {
  name: string;
  type: NodeSchemaType;
  package: Package;

  constructor(raw: RawNodeSchema, pkg: Package) {
    this.name = raw.name;
    this.type = raw.type;
    this.package = pkg;
    makeAutoObservable(this);
  }
}

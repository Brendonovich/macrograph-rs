import { makeAutoObservable, toJS } from "mobx";
import { Position } from "@macrograph/core-types";

import { Graph, Node, Pin } from "~/models";

class UIStore {
  selectedNode: Node | null = null;
  draggingPin: Pin | null = null;
  hoveringPin: Pin | null = null;
  mouseDragLocation: XY | null = null;
  schemaMenuPosition: Position | null = null;
  mouseDownLocation: XY | null = null;
  mouseDownTranslate: XY | null = null;

  currentGraph: Graph | null = null;

  graphOffset: XY = {
    x: 0,
    y: 0,
  };
  translate: XY = {
    x: 0,
    y: 0,
  };
  scale = 1;

  pinPositions: WeakMap<Pin, XY> = new Map();

  constructor() {
    makeAutoObservable(this);
  }

  toGraphSpace(point: XY) {
    return {
      x: point.x / this.scale + this.translate.x,
      y: point.y / this.scale + this.translate.y,
    };
  }

  // Converts a location in the graph (eg the graph's origin (0,0)) to its location on screen
  toScreenSpace(point: XY) {
    return {
      x: (point.x - this.translate.x) * this.scale,
      y: (point.y - this.translate.y) * this.scale,
    };
  }

  setSelectedNode(node?: Node) {
    this.selectedNode?.setSelected(false);
    this.selectedNode = node ?? null;
    this.selectedNode?.setSelected(true);
  }

  setPinPosition(pin: Pin, position: XY) {
    this.pinPositions.set(pin, position);
  }

  updateTranslate(delta: XY) {
    this.translate.x += delta.x;
    this.translate.y += delta.y;
  }

  setTranslate(translate: XY) {
    this.translate = translate;
  }

  updateScale(delta: number, screenOrigin: XY) {
    const startGraphOrigin = UI.toGraphSpace(screenOrigin);
    this.scale = Math.min(Math.max(1, this.scale + delta), 2.5);
    const endGraphOrigin = this.toScreenSpace(startGraphOrigin);

    this.translate = {
      x: this.translate.x + (endGraphOrigin.x - screenOrigin.x) / this.scale,
      y: this.translate.y + (endGraphOrigin.y - screenOrigin.y) / this.scale,
    };
  }

  setDraggingPin(pin?: Pin) {
    this.draggingPin = pin ?? null;
  }
  setHoveringPin(pin?: Pin) {
    this.hoveringPin = pin ?? null;
  }

  setMouseDragLocation(location?: XY) {
    this.mouseDragLocation = location ?? null;
  }

  setMouseDownLocation(location?: XY) {
    this.mouseDownLocation = location ?? null;
  }

  setSchemaMenuPosition(position?: XY) {
    this.schemaMenuPosition = position ?? null;
  }

  setMouseDownTranslate(translate?: XY) {
    this.mouseDownTranslate = translate ?? null;
  }

  setGraphOffset(offset: XY) {
    this.graphOffset = offset;
  }

  setCurrentGraph(graph: Graph) {
    this.setSchemaMenuPosition()
    this.currentGraph = graph;
  }
}

export const UI = new UIStore();

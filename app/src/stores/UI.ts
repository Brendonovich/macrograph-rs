import { makeAutoObservable } from "mobx";
import {Position} from "@macrograph/core-types"

import { Node, Pin } from "~/models";

class UIStore {
  selectedNode: Node | null = null;
  draggingPin: Pin | null = null;
  hoveringPin: Pin | null = null;
  mouseDragLocation: XY | null = null;
  schemaMenuPosition: Position | null = null;

  translate: XY = {
    x: 0,
    y: 0,
  };
  scale = 1;

  pinPositions: WeakMap<Pin, XY> = new Map();

  constructor() {
    makeAutoObservable(this);
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

  updateScale(delta: number, origin: XY) {
    const initialTranslate = { ...this.translate };
    const initialScale = this.scale;

    this.setTranslate({
      x: origin.x / initialScale + this.translate.x,
      y: origin.y / initialScale + this.translate.y,
    });
    this.scale = Math.min(Math.max(1, this.scale + delta), 5);
    this.setTranslate({
      x: initialTranslate.x + (origin.x / initialScale - origin.x / UI.scale),
      y: initialTranslate.y + (origin.y / initialScale - origin.y / UI.scale),
    });
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

  setSchemaMenuPosition(position?: XY) {
    this.schemaMenuPosition = position ?? null;
  }
}

export const UI = new UIStore();

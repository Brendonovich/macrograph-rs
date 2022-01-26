import {
  useCallback,
  useEffect,
  useLayoutEffect,
  useRef,
  useState,
} from "react";
import { autorun, runInAction } from "mobx";

import { Pin } from "~/models";
import { UI } from "~/stores";
import { useCurrentGraph } from "~/contexts";
import { pinIsInput, pinIsOutput, pinsCanConnect } from "~/utils";

export const usePin = (pin: Pin) => {
  const ref = useRef<HTMLDivElement>(null);
  const graph = useCurrentGraph();

  const [mouseState, setMouseState] = useState({
    hovering: UI.hoveringPin === pin,
    dragging: UI.draggingPin === pin,
  });

  useEffect(() => {
    const dispose = autorun(() =>
      setMouseState({
        hovering: UI.hoveringPin === pin,
        dragging: UI.draggingPin === pin,
      })
    );

    return () => dispose();
  }, [pin]);

  const handleMouseDrag = useCallback((e: MouseEvent) => {
    UI.setDraggingPin(pin);
    UI.setMouseDragLocation({
      x: e.clientX,
      y: e.clientY,
    });
  }, []);

  const handleMouseUp = useCallback(() => {
    UI.setDraggingPin();
    window.removeEventListener("mouseup", handleMouseUp);
    window.removeEventListener("mousemove", handleMouseDrag);
  }, [handleMouseDrag]);

  useEffect(() => {
    let justMouseUpped = false;
    ref.current?.addEventListener("mouseover", () => {
      if (
        !UI.draggingPin ||
        (pinIsOutput(UI.draggingPin) &&
          pinIsInput(pin) &&
          pinsCanConnect(UI.draggingPin, pin)) ||
        (pinIsOutput(pin) &&
          pinIsInput(UI.draggingPin) &&
          pinsCanConnect(pin, UI.draggingPin))
      )
        UI.setHoveringPin(pin);
    });
    ref.current?.addEventListener("mouseleave", () => {
      if (justMouseUpped) return;
      UI.setHoveringPin();
    });
    ref.current?.addEventListener("mouseup", (e) => {
      runInAction(() => {
        console.log(1)
        // Necessary since safari fires 'mouseleave' just after mouseup. i hate this.
        justMouseUpped = true;
        setTimeout(() => (justMouseUpped = false), 1);
        UI.setHoveringPin(pin);

        if (!UI.draggingPin || UI.draggingPin === pin) return;

        if (pinIsOutput(pin) && pinIsInput(UI.draggingPin))
          graph.connectPins(pin, UI.draggingPin);
        else if (pinIsInput(pin) && pinIsOutput(UI.draggingPin))
          graph.connectPins(UI.draggingPin, pin);

        UI.setDraggingPin();
      });
    });
    ref.current?.addEventListener("mousedown", () => {
      window.addEventListener("mouseup", handleMouseUp);
      window.addEventListener("mousemove", handleMouseDrag);
    });
    ref.current?.addEventListener("dblclick", () => {
      pin.disconnect();
    });
  }, [pin]);

  useLayoutEffect(() => {
    let rect = ref.current?.getBoundingClientRect();

    if (rect)
      UI.setPinPosition(pin, {
        x: rect.x + rect.width / 2,
        y: rect.y + rect.height / 2,
      });
  });

  return {
    ref,
    active: mouseState.hovering || mouseState.dragging,
  };
};

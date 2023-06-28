export class Input {
  private static keys: Map<string, boolean> = new Map();
  private static mouseX: number = 0;
  private static mouseY: number = 0;
  private static wheel: number = 0;
  private static mouseDown: boolean = false;

  public static init() {
    window.addEventListener("keydown", (e) => {
      Input.keys.set(e.key, true);
    });

    window.addEventListener("keyup", (e) => {
      Input.keys.set(e.key, false);
    });

    window.addEventListener("mousemove", (e) => {
      Input.mouseX = e.clientX;
      Input.mouseY = e.clientY;
    });

    window.addEventListener("mousedown", () => {
      Input.mouseDown = true;
    });

    window.addEventListener("mouseup", () => {
      Input.mouseDown = false;
    });

    window.addEventListener("wheel", (e) => {
      Input.wheel = e.deltaY;
    });
  }

  public static isMouseDown(): boolean {
    return Input.mouseDown;
  }

  public static getMouseX(): number {
    return Input.mouseX;
  }

  public static getMouseY(): number {
    return Input.mouseY;
  }

  public static getWheel(): number {
    const wheel = Input.wheel;
    Input.wheel = 0;
    return wheel;
  }

  public static isDown(key: string): boolean {
    return Input.keys.get(key) || false;
  }
}

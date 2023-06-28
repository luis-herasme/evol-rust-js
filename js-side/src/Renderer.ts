export class Renderer {
  private ctx: CanvasRenderingContext2D;

  constructor() {
    const canvas = document.createElement("canvas");

    if (!canvas) {
      throw new Error("Could not create canvas");
    }

    const ctx = canvas.getContext("2d");

    if (!ctx) {
      throw new Error("Could not get canvas context");
    }

    this.ctx = ctx;

    canvas.width = window.innerWidth;
    canvas.height = window.innerHeight;

    document.body.appendChild(canvas);
  }

  public moveCamera(x: number, y: number) {
    this.ctx.translate(x, y);
  }

  public zoomCamera(zoom: number, clientX: number, clientY: number) {
    const transform = this.ctx.getTransform();
    // Take into account the translation of the camera and the zoom
    const x = (clientX - transform.e) / transform.a;
    const y = (clientY - transform.f) / transform.d;

    this.ctx.translate(x, y);

    this.ctx.scale(zoom, zoom);

    this.ctx.translate(-x, -y);
  }

  public getScale(): number {
    return this.ctx.getTransform().a;
  }

  public getCameraX(): number {
    return this.ctx.getTransform().e;
  }

  public getCameraY(): number {
    return this.ctx.getTransform().f;
  }

  public clear(color: string = "black") {
    this.ctx.save();
    this.ctx.setTransform(1, 0, 0, 1, 0, 0);
    this.ctx.fillStyle = color;
    this.ctx.fillRect(0, 0, this.ctx.canvas.width, this.ctx.canvas.height);
    this.ctx.restore();
  }

  public circle(x: number, y: number, radius: number, color: string) {
    this.ctx.beginPath();
    this.ctx.arc(x, y, radius, 0, 2 * Math.PI, false);
    this.ctx.strokeStyle = "white";
    this.ctx.fillStyle = color;
    this.ctx.stroke();
    this.ctx.fill();
  }

  public rect(
    x: number,
    y: number,
    width: number,
    height: number,
    color: string
  ) {
    this.ctx.fillStyle = color;
    this.ctx.fillRect(x - width / 2, y - height / 2, width, height);
  }
}

/** Turns a title into a URL slug: lowercase words joined by `-`. */
export function slugify(title: string): string {
  return title
    .split(/[^\p{L}\p{N}]+/u)
    .filter((w) => w !== "")
    .map((w) => w.toLowerCase())
    .join("-");
}

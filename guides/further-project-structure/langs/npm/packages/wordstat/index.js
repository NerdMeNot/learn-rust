import { normalize } from "./normalize.js";

export function count(text) {
  const counts = {};
  for (const w of text.split(/\s+/).filter(Boolean)) {
    const k = normalize(w);
    counts[k] = (counts[k] ?? 0) + 1;
  }
  return counts;
}

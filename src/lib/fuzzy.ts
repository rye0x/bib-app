// A small subsequence fuzzy matcher for the quick-open finder. Not trying to
// be fzf — just good enough to rank a project's file list as the user types.
//
// `fuzzyMatch` returns null when `query` isn't a subsequence of `text`, or a
// score plus the matched character indices (for highlighting). Higher score is
// a better match: consecutive runs, matches at word boundaries, and matches
// near the start all score higher.

export type FuzzyMatch = { score: number; positions: number[] };

export function fuzzyMatch(query: string, text: string): FuzzyMatch | null {
  if (query === "") return { score: 0, positions: [] };

  const q = query.toLowerCase();
  const t = text.toLowerCase();
  const positions: number[] = [];

  let score = 0;
  let qi = 0;
  let prevMatch = -2; // index of the previous matched char, for run detection

  for (let ti = 0; ti < t.length && qi < q.length; ti++) {
    if (t[ti] !== q[qi]) continue;

    let bonus = 1;
    if (ti === prevMatch + 1) bonus += 4; // consecutive run
    if (ti === 0 || /[\s/\\._-]/.test(t[ti - 1])) bonus += 3; // word boundary
    if (ti < 3) bonus += 1; // near the start

    score += bonus;
    positions.push(ti);
    prevMatch = ti;
    qi++;
  }

  if (qi < q.length) return null; // not all query chars matched

  // Prefer shorter targets when scores are otherwise close.
  score -= text.length * 0.05;
  return { score, positions };
}

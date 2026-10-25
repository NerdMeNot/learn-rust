// arrays are passed by reference, so the function can change the caller's array
function addBonus(scores: number[], bonus: number): void {
  for (let i = 0; i < scores.length; i++) {
    scores[i] += bonus;
  }
  scores.push(0);
}

const scores = [10, 20];
addBonus(scores, 5);
console.log(scores);

function solve(input) {
  const gridSize = 300

  var grid = Array(gridSize).fill(0).map(a => Array(gridSize).fill(0));
  for (var i = 1; i <= gridSize; i++) {
    for (var j = 1; j <= gridSize; j++) {
      grid[i -1][j-1] = parseInt(("000" + ((((i+10) * j) + input) * (i+10))).split("").reverse()[2]) - 5;
    }
  }

  var max = {t: 0, x:0, y: 0, g: 0}
  var totals = Array(gridSize).fill(Array(gridSize).fill(0));
  var subgrid = Array(gridSize).fill(0).map(a => Array(gridSize).fill(0));
  for (var g = 0; g < gridSize; g++) {
    for (var i = 1; i < gridSize - 1; i++) {
      for (var j = 2; j < gridSize - 1; j++) {
        for (x = i; x < i + g && x < gridSize && (g + j-1) < gridSize; x++) {
          subgrid[i][j] += grid[x][j+g-1];
        }
        for (x = j; x < j+g-1 && x < gridSize && (g+i-1) < gridSize; x++) {
          subgrid[i][j] += grid[i+g-1][x];
        }
        if (subgrid[i][j] > max.t) {
          max = {t: subgrid[i][j], x: i, y: j, g: g}
        }
      }
    }
  }
  return `${max.x + 1},${max.y + 1},${max.g}`;
}

console.log(solve(1788))

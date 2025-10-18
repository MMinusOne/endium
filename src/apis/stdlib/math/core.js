const Math = {
  max(...nums) {
    let biggest;
    for (const num of nums) {
      if (!biggest || num > biggest) biggest = num;
    }

    return biggest;
  },
  min(...nums) {
    let smallest;
    for (const num of nums) {
      if (!smallest || num < smallest) smallest = num;
    }

    return smallest;
  },
  pow(base, exponent) {
    return base ** exponent;
  },
};

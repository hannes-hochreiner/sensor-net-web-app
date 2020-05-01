<template>
  <svg height="100%" width="100%" viewBox="0 0 460 240"  preserveAspectRatio="xMidYMid meet">
    <defs>
      <linearGradient id="myGradient" gradientTransform="rotate(90)">
        <stop offset="5%"  stop-color="gold" />
        <stop offset="95%" stop-color="red" />
      </linearGradient>
    </defs>
  </svg>
</template>

<script>
import * as d3 from 'd3';
export default {
  name: 'line-chart',
  data() {
    return {
      data1: [
        [
          {date: new Date('2020-04-01T12:00:00'), value: 10},
          {date: new Date('2020-04-01T12:03:00'), value: 12},
          {date: new Date('2020-04-01T12:05:00'), value: 7},
          {date: new Date('2020-04-01T12:20:00'), value: 16},
          {date: new Date('2020-04-01T12:23:00'), value: 9}
        ], [
          {date: new Date('2020-04-01T12:00:00'), value: 3},
          {date: new Date('2020-04-01T12:01:00'), value: 10},
          {date: new Date('2020-04-01T12:07:00'), value: 6},
          {date: new Date('2020-04-01T12:14:00'), value: 12},
          {date: new Date('2020-04-01T12:20:00'), value: 5}
        ]
      ]
    };
  },
  mounted() {
    this.calculatePath();
  },
  methods: {
    getScales() {
      const x = d3.scaleTime().range([0, 430]);
      const y = d3.scaleLinear().range([210, 0]);
      d3.axisLeft().scale(x);
      d3.axisBottom().scale(y);
      x.domain(d3.extent(this.data1.flat(), d => d.date));
      y.domain([0, d3.max(this.data1.flat(), d => d.value)]);
      return { x, y };
    },
    calculatePath() {
      const scale = this.getScales();
      let line = d3.line().curve(d3.curveCatmullRom.alpha(1))
        .x(function(d) { return scale.x(d.date) })
        .y(function(d) { return scale.y(d.value) });
      d3.select('svg').append("g").attr("transform", `translate(30, 10)`).selectAll(".test").data(this.data1).enter().append("path").attr("class", 'test').attr("d", line);
      d3.select("svg").append("g").attr("transform", `translate(30, 10)`).call(d3.axisLeft(scale.y));
      d3.select("svg").append("g").attr("transform", `translate(30,220)`).call(d3.axisBottom(scale.x));
    },
  },
};
</script>
<style scoped>
svg {
  background-color: #aaaaaa;
}
svg >>> path.test {
  fill: none;
  stroke: url(#myGradient);
  stroke-width: 3px;
}
</style>

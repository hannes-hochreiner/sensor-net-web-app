<template>
  <svg height="100%" width="100%" viewBox="0 0 460 240"  preserveAspectRatio="xMidYMid meet">
    <g transform="translate(50, 10)"/>
    <g transform="translate(50, 10)"/>
    <g transform="translate(50,220)"/>
  </svg>
</template>

<script>
import * as d3 from 'd3';
export default {
  name: 'line-chart',
  props: {
    data: Array
  },
  mounted() {
    this.calculatePath();
  },
  computed: {
    transformedData: function() {
      return this.data.map(elem => {
        elem.ts = new Date(elem.ts);

        return elem;
      });
    }
  },
  watch: {
    transformedData: function() {
      this.calculatePath();
    }
  },
  methods: {
    calculatePath() {
      const x = d3.scaleTime().range([0, 390]);
      const y = d3.scaleLinear().range([210, 0]);
      d3.axisLeft().scale(x);
      d3.axisBottom().scale(y);
      x.domain(d3.extent(this.transformedData.flat(), d => d.ts));
      y.domain(d3.extent(this.transformedData.flat(), d => d.val));
      let line = d3.line()
        // .curve(d3.curveCatmullRom.alpha(1))
        .x(function(d) { return x(d.ts) })
        .y(function(d) { return y(d.val) });
      // multi-line
      // d3.select(this.$el.childNodes[0]).selectAll(".data").data(this.transformedData).enter().append("path").attr("class", "data").attr("d", line);
      d3.select(this.$el.childNodes[0]).append("path").datum(this.transformedData).attr("class", "data").attr("d", line);
      d3.select(this.$el.childNodes[1]).call(d3.axisLeft(y));
      d3.select(this.$el.childNodes[2]).call(d3.axisBottom(x));
    },
  },
};
</script>
<style scoped>
svg {
  background-color: #f5f5f5;
}
svg >>> path.data {
  fill: none;
  stroke: #000000;
  stroke-width: 2px;
}
</style>

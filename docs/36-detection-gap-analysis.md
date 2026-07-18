# Detection gap analysis

Tyraxes engagements should end with defender value, not only offense trophies.

## For each confirmed path

Record:

- telemetry that *did* fire
- telemetry that *should* have fired
- blind spots (no sensor, wrong tier, retention gap)
- suggested detections / hunts

## Where to put it

- per finding: `Detection ideas`
- rollup: `.tyraxes/redteam/report/detection-gaps.md`

## Useful operator prompt

```text
For every confirmed edge in attack-graph.md, write detection gaps with
data sources, approximate query logic, and expected noise.
```

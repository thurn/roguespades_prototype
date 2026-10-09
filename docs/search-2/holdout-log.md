# Holdout and final seed uses

Every measurement on holdout or final seeds, appended by `rsa.itcli`. A holdout block retires after 20 uses.

| When | Seeds | Env | Measurement | Result | Commit |
| --- | --- | --- | --- | --- | --- |
| 2026-10-08 23:18 | holdout block 1 | holdout0-t1-wp | compare search1-L vs p2-alive-L | +3.88 [+2.79, +5.08] | 2ea57bf |
| 2026-10-08 23:25 | holdout block 1 | holdout0-t2-wp | compare search1-L vs p2-alive-L | +6.12 [+4.13, +8.22] | 2ea57bf |
| 2026-10-09 00:22 | holdout block 1 | holdout0-t1-wp | compare p2-alive-L vs p4ab | +0.79 [-0.30, +1.64] | f1a73b2 |
| 2026-10-09 00:22 | holdout block 1 | holdout0-t1-wp | compare p2-alive-L vs p4e-L | +1.75 [+0.83, +2.57] | f1a73b2 |
| 2026-10-09 00:43 | holdout block 1 | holdout0-t2-wp | compare search1-L vs p2-alive-L | +6.12 [+4.13, +8.22] | 3f615b5 |
| 2026-10-09 00:43 | holdout block 1 | holdout0-t2-wp | compare search1-L vs p4h | +7.78 [+6.58, +9.84] | 3f615b5 |
| 2026-10-09 01:23 | holdout block 1 | holdout0-t1-wp | compare p2-alive-L vs p4i-L | +1.37 [-3.90, +5.62] | 191b1bf |
| 2026-10-09 01:29 | holdout block 1 | holdout0-t1-wp | compare p4e-L vs p4i-L | -0.05 [-0.93, +0.84] | 56ea259 |
| 2026-10-09 01:50 | holdout block 1 | holdout0-t1-wp | compare p4i-L vs p4ic | +0.33 [+0.00, +1.00] | bd759e7 |
| 2026-10-09 01:56 | holdout block 1 | holdout0-t1-wp | compare p4i-L vs r4-prices-c25 | +1.89 [+1.05, +2.74] | bd759e7 |
| 2026-10-09 02:03 | holdout block 1 | holdout0-t2-wp | compare p4i-L vs r4-prices-c25 | +1.75 [+0.17, +2.99] | bceb83f |
| 2026-10-09 02:21 | holdout block 1 | holdout0-t1-wp | compare r4-prices-c25 vs r5-slots7 | +0.92 [-0.03, +1.87] | f18d21a |
| 2026-10-09 02:21 | holdout block 1 | holdout0-t1-wp | compare r4-prices-c25 vs r5-flatset | +1.73 [+0.89, +2.52] | f18d21a |
| 2026-10-09 02:22 | holdout block 1 | holdout0-t1-wp | compare r4-prices-c25 vs r5-s7-flat | +1.64 [+0.77, +2.38] | f18d21a |
| 2026-10-09 02:30 | holdout block 1 | holdout0-t2-wp | compare r4-prices-c25 vs r5-s7-flat | +0.75 [-0.48, +2.32] | f18d21a |
| 2026-10-09 02:30 | holdout block 1 | holdout0-t2-wp | compare r4-prices-c25 vs r5-flatset | -0.05 [-1.14, +1.50] | f18d21a |
| 2026-10-09 03:20 | final block 1 | final0-t1-wp | phase 5 measure base-game | 71.4 [69.4, 76.0] | 558f849 |
| 2026-10-09 03:20 | final block 1 | final0-t1-wp | phase 5 measure rec3 | 85.7 [83.5, 92.2] | 558f849 |
| 2026-10-09 03:20 | final block 1 | final0-t1-wp | phase 5 measure ru3-sym | 87.8 [85.9, 94.5] | 558f849 |
| 2026-10-09 03:20 | final block 1 | final0-t1-wp | phase 5 measure ru3-slots8 | 89.0 [85.6, 95.6] | 558f849 |
| 2026-10-09 03:20 | final block 1 | final0-t1-wp | phase 5 measure search1 | 78.4 [76.8, 81.2] | 558f849 |
| 2026-10-09 03:20 | final block 1 | final0-t1-wp | phase 5 measure search1-L | 73.8 [72.8, 74.8] | 558f849 |
| 2026-10-09 03:22 | final block 1 | final0-t1-wp | phase 5 compare base-game vs rec3 | +14.25 [+9.82, +20.71] | 558f849 |
| 2026-10-09 03:22 | final block 1 | final0-t1-wp | phase 5 compare base-game vs ru3-sym | +16.36 [+11.79, +23.00] | 558f849 |
| 2026-10-09 03:22 | final block 1 | final0-t1-wp | phase 5 compare base-game vs ru3-slots8 | +17.57 [+11.95, +24.18] | 558f849 |
| 2026-10-09 03:22 | final block 1 | final0-t1-wp | phase 5 compare base-game vs search1 | +6.97 [+2.61, +9.57] | 558f849 |
| 2026-10-09 03:22 | final block 1 | final0-t1-wp | phase 5 compare base-game vs search1-L | +2.40 [-2.22, +4.19] | 558f849 |
| 2026-10-09 03:53 | final block 1 | final1-t1-wp | phase 5 measure base-game | 70.4 [68.7, 77.7] | 2264804 |
| 2026-10-09 03:53 | final block 1 | final1-t1-wp | phase 5 measure rec3 | 82.3 [81.9, 88.0] | 2264804 |
| 2026-10-09 03:53 | final block 1 | final1-t1-wp | phase 5 measure ru3-sym | 82.9 [82.3, 90.5] | 2264804 |
| 2026-10-09 03:53 | final block 1 | final1-t1-wp | phase 5 measure ru3-slots8 | 87.4 [84.9, 93.5] | 2264804 |
| 2026-10-09 03:53 | final block 1 | final1-t1-wp | phase 5 measure search1 | 80.2 [77.9, 84.1] | 2264804 |
| 2026-10-09 03:53 | final block 1 | final1-t1-wp | phase 5 measure search1-L | 72.5 [72.0, 76.0] | 2264804 |
| 2026-10-09 03:54 | final block 1 | final1-t1-wp | phase 5 compare base-game vs rec3 | +11.90 [+5.18, +16.42] | 6fb9e1c |
| 2026-10-09 03:54 | final block 1 | final1-t1-wp | phase 5 compare base-game vs ru3-sym | +12.46 [+7.27, +18.22] | 6fb9e1c |
| 2026-10-09 03:54 | final block 1 | final1-t1-wp | phase 5 compare base-game vs ru3-slots8 | +16.96 [+9.90, +21.97] | 6fb9e1c |
| 2026-10-09 03:54 | final block 1 | final1-t1-wp | phase 5 compare base-game vs search1 | +9.75 [+2.95, +13.19] | 6fb9e1c |
| 2026-10-09 03:54 | final block 1 | final1-t1-wp | phase 5 compare base-game vs search1-L | +2.10 [-4.80, +5.07] | 6fb9e1c |
| 2026-10-09 04:25 | final block 1 | final2-t1-wp | phase 5 measure base-game | 70.2 [68.7, 74.9] | b65428a |
| 2026-10-09 04:25 | final block 1 | final2-t1-wp | phase 5 measure rec3 | 86.9 [83.0, 92.5] | b65428a |
| 2026-10-09 04:25 | final block 1 | final2-t1-wp | phase 5 measure ru3-sym | 81.6 [81.4, 86.1] | b65428a |
| 2026-10-09 04:25 | final block 1 | final2-t1-wp | phase 5 measure ru3-slots8 | 83.1 [83.3, 91.0] | b65428a |
| 2026-10-09 04:25 | final block 1 | final2-t1-wp | phase 5 measure search1 | 78.0 [76.0, 82.2] | b65428a |
| 2026-10-09 04:25 | final block 1 | final2-t1-wp | phase 5 measure search1-L | 73.0 [72.4, 75.5] | b65428a |
| 2026-10-09 04:26 | final block 1 | final2-t1-wp | phase 5 compare base-game vs rec3 | +16.68 [+10.52, +21.26] | b65428a |
| 2026-10-09 04:26 | final block 1 | final2-t1-wp | phase 5 compare base-game vs ru3-sym | +11.41 [+7.95, +15.78] | b65428a |
| 2026-10-09 04:26 | final block 1 | final2-t1-wp | phase 5 compare base-game vs ru3-slots8 | +12.88 [+10.44, +20.42] | b65428a |
| 2026-10-09 04:26 | final block 1 | final2-t1-wp | phase 5 compare base-game vs search1 | +7.74 [+3.32, +11.48] | b65428a |
| 2026-10-09 04:26 | final block 1 | final2-t1-wp | phase 5 compare base-game vs search1-L | +2.76 [-2.00, +5.33] | b65428a |
| 2026-10-09 04:51 | final block 1 | final0-t2-wp | phase 5 measure base-game | 69.3 [66.9, 73.8] | b65428a |
| 2026-10-09 04:51 | final block 1 | final0-t2-wp | phase 5 measure rec3 | 85.1 [82.6, 91.2] | b65428a |
| 2026-10-09 04:51 | final block 1 | final0-t2-wp | phase 5 measure ru3-sym | 87.3 [85.1, 94.2] | b65428a |
| 2026-10-09 04:51 | final block 1 | final0-t2-wp | phase 5 measure ru3-slots8 | 89.8 [85.7, 96.2] | b65428a |
| 2026-10-09 04:51 | final block 1 | final0-t2-wp | phase 5 measure search1 | 78.4 [76.6, 81.2] | b65428a |
| 2026-10-09 04:51 | final block 1 | final0-t2-wp | phase 5 measure search1-L | 74.2 [72.5, 75.4] | b65428a |
| 2026-10-09 04:54 | final block 1 | final0-t2-wp | phase 5 compare base-game vs rec3 | +15.77 [+11.22, +22.22] | b65428a |
| 2026-10-09 04:54 | final block 1 | final0-t2-wp | phase 5 compare base-game vs ru3-sym | +17.97 [+13.20, +24.60] | b65428a |
| 2026-10-09 04:54 | final block 1 | final0-t2-wp | phase 5 compare base-game vs ru3-slots8 | +20.56 [+13.96, +26.57] | b65428a |
| 2026-10-09 04:54 | final block 1 | final0-t2-wp | phase 5 compare base-game vs search1 | +9.06 [+4.32, +12.07] | b65428a |
| 2026-10-09 04:54 | final block 1 | final0-t2-wp | phase 5 compare base-game vs search1-L | +4.89 [-0.31, +7.17] | b65428a |
| 2026-10-09 05:18 | final block 1 | final1-t2-wp | phase 5 measure base-game | 72.2 [70.3, 79.4] | 2c6308c |
| 2026-10-09 05:18 | final block 1 | final1-t2-wp | phase 5 measure rec3 | 82.7 [82.2, 88.6] | 2c6308c |
| 2026-10-09 05:18 | final block 1 | final1-t2-wp | phase 5 measure ru3-sym | 83.3 [82.8, 91.0] | 2c6308c |
| 2026-10-09 05:18 | final block 1 | final1-t2-wp | phase 5 measure ru3-slots8 | 88.7 [85.7, 94.2] | 2c6308c |
| 2026-10-09 05:18 | final block 1 | final1-t2-wp | phase 5 measure search1 | 80.1 [77.5, 84.0] | 2c6308c |
| 2026-10-09 05:18 | final block 1 | final1-t2-wp | phase 5 measure search1-L | 71.6 [70.6, 75.0] | 2c6308c |
| 2026-10-09 05:21 | final block 1 | final1-t2-wp | phase 5 compare base-game vs rec3 | +10.50 [+4.37, +15.73] | 2c6308c |
| 2026-10-09 05:21 | final block 1 | final1-t2-wp | phase 5 compare base-game vs ru3-sym | +11.11 [+6.03, +17.64] | 2c6308c |
| 2026-10-09 05:21 | final block 1 | final1-t2-wp | phase 5 compare base-game vs ru3-slots8 | +16.49 [+9.27, +22.56] | 2c6308c |
| 2026-10-09 05:21 | final block 1 | final1-t2-wp | phase 5 compare base-game vs search1 | +7.95 [-0.25, +11.81] | 2c6308c |
| 2026-10-09 05:21 | final block 1 | final1-t2-wp | phase 5 compare base-game vs search1-L | -0.62 [-7.50, +2.89] | 2c6308c |
| 2026-10-09 05:46 | final block 1 | final2-t2-wp | phase 5 measure base-game | 71.1 [69.8, 76.1] | 2c6308c |
| 2026-10-09 05:46 | final block 1 | final2-t2-wp | phase 5 measure rec3 | 88.2 [84.5, 93.5] | 2c6308c |
| 2026-10-09 05:46 | final block 1 | final2-t2-wp | phase 5 measure ru3-sym | 81.8 [81.4, 86.4] | 2c6308c |
| 2026-10-09 05:46 | final block 1 | final2-t2-wp | phase 5 measure ru3-slots8 | 84.2 [84.0, 92.8] | 2c6308c |
| 2026-10-09 05:46 | final block 1 | final2-t2-wp | phase 5 measure search1 | 78.9 [76.1, 82.8] | 2c6308c |
| 2026-10-09 05:46 | final block 1 | final2-t2-wp | phase 5 measure search1-L | 73.1 [71.9, 75.9] | 2c6308c |
| 2026-10-09 05:49 | final block 1 | final2-t2-wp | phase 5 compare base-game vs rec3 | +17.09 [+10.32, +22.03] | 2c6308c |
| 2026-10-09 05:49 | final block 1 | final2-t2-wp | phase 5 compare base-game vs ru3-sym | +10.75 [+5.97, +14.72] | 2c6308c |
| 2026-10-09 05:49 | final block 1 | final2-t2-wp | phase 5 compare base-game vs ru3-slots8 | +13.13 [+9.58, +20.37] | 2c6308c |
| 2026-10-09 05:49 | final block 1 | final2-t2-wp | phase 5 compare base-game vs search1 | +7.82 [+2.04, +11.00] | 2c6308c |
| 2026-10-09 05:49 | final block 1 | final2-t2-wp | phase 5 compare base-game vs search1-L | +2.00 [-3.45, +5.03] | 2c6308c |

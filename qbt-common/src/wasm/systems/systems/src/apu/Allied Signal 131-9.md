# Allied Signal 131-9
This apu code is based on the FlyByWire APS3200, but we have taken into consideration the OAT to have a lineal impact on the EGT but THIS IS NOT A 1/1 representation to the real impact that it may have on the real EGT so it's just an aproximation. We've been able to do so because at the moment we are running an oversimplified version of their code only taking into consideration time values and not N as we don't have the option to recavate that information. FLyByWire also suspects that there is an altitude EGT effect (less air density causing higher EGT during start as it provides less cooling?), but that effect is also unknown by them and us.

The data used for this apu is taken from:[Video][https://www.youtube.com/watch?v=SlrTGDHcpcQ]
Curve fitted polynomial regression generation: http://polynomialregression.drque.net/online.php

## Time to EGT
EGT is measured in Celsius
```
seconds, EGT
0,0;//1:07
1,0;08
2,0;09
3,0;//1:10
4,0;011
5,
6,
7
```



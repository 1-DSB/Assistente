import math

valores = [3,0,1]
expo = []
for i in valores:
    expo.append(math.exp(i))

soma = sum(expo)

probas = []

for i in expo:
    probas.append(i/soma)

print(sum(probas))
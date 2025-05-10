#!/usr/bin/python

import cmath
tau = 2 * cmath.pi

def average(readings):
	base = cmath.e ** (1j * tau / 360)
	total = 0
	for r in readings:
		v = r[1] * base ** r[0]
		total += v
	result = total / len(readings)
	return (cmath.log(result, base).real, abs(result))

print(average(((12, 1), (15, 1), (13, 1), (9, 1), (16, 1))))
print(average(((358, 1), (1, 1), (359, 1), (355, 1), (2, 1))))
print(average(((210, 1), (290, 1), (10, 1), (90, 1), (170, 1))))
# hash-to-curve
The Pedersen commitment scheme involves two key operations:

1.	Commitment Generation:
  Given a secret value x and a randomly chosen scalar r, the commitment is generated as C = r*G + x*H, where:
	G is a generator point on the elliptic curve.
	H is another generator point, known as the commitment base or "hiding" point. This point is derived from a hash function.

2.	Commitment Verification:

To verify the commitment, one needs to confirm that C is indeed a valid commitment to some value x. This is done by checking whether C is on the curve and satisfies the equation C = r*G + x*H, where r and H are publicly known.

-module(darts).

-export([score/2]).

score(R) when R =< 1 -> 10;
score(R) when R =< 5 -> 5;
score(R) when R =< 10 -> 1;
score(_) -> 0.

score(X, Y) -> score((X * X + Y * Y) / 10).

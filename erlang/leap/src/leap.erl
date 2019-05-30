-module(leap).

-export([leap_year/1]).


is_divisible(X, Y) -> X rem Y == 0.

leap_year(Year) -> is_divisible(Year, 4) and not is_divisible(Year, 100) or is_divisible(Year, 400).

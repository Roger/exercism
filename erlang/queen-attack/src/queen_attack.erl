-module(queen_attack).

-export([can_attack/2]).

% test same row
can_attack({X1, _Y1}, {X2, _Y2}) when X1 == X2-> true;
% test same col
can_attack({_X1, Y1}, {_X2, Y2}) when Y1 == Y2-> true;
% test diagonal
can_attack({X1, Y1}, {X2, Y2})-> abs(X1 - X2) == abs(Y1 - Y2).

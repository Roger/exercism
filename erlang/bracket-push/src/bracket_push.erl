-module(bracket_push).

-export([is_paired/1]).

is_paired(String) -> is_paired(String, []).

% get pair of a close bracket
get_pair(CHR) when CHR == $] -> $[;
get_pair(CHR) when CHR == $} -> ${;
get_pair(CHR) when CHR == $) -> $(.

% add opens to a list
is_paired([H | T], Opens) when H == ${; H == $[; H == $( ->
	is_paired(T, [H | Opens]);

% if open is empty, it's unbalanced
is_paired([H | _T], []) when H == $}; H == $]; H == $) -> false;
% check if the close have an open and keep consuming
is_paired([H | T], [OH | OT]) when H == $}; H == $]; H == $) ->
	PAIR = get_pair(H),
	if 
		OH /= PAIR -> false;
		true -> is_paired(T, OT)
	end;

% consume non brackets
is_paired([_H | T], Opens) -> is_paired(T, Opens);

% when everything is consumed from opens and string, means it's balanced
is_paired([], []) -> true;
% if opens is not empty when finish consuming the string, it's unbalanced
is_paired([], _) -> false.

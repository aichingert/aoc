Day15:
begin
    integer procedure MOD (a, b);
        integer a, b;
    begin
        integer out;
        out := a;

    loop:
        if out >= b then 
        begin
            out := out - b;
            go to loop
        end;

        MOD := out
    end MOD;

    procedure PART ONE (incr, times);
        integer incr, times;
    begin
        integer i, j, cal, loc;
        integer array S[0:times + 1];

        for i := 1 step 1 until times do
            begin
                loc := MOD ( (loc + incr + 1), i );

                for j := loc + 1 step 1 until i do
                    begin
                        cal := j - (loc + 1);
                        S[i - cal] := S[i - cal - 1]
                    end;

                S[loc + 1] := i
            end;

        outstring(1, "Part one: ");
        outinteger(1, S[MOD ( loc + 2, times )]);
        outstring(1, "\n")

    end PART ONE;

    procedure PART TWO (incr, times);
        integer incr, times;
    begin
        integer i, loc, ans;

        for i := 1 step 1 until times do
            begin
                loc := MOD ( loc + incr + 1, i );

                if loc = 0 then ans := i
            end;

        outstring(1, "Part two: ");
        outinteger(1, ans);
        outstring(1, "\n")
    end PART TWO;

    procedure MAIN;
    begin
        PART ONE (3, 2017);
        PART TWO (3, 50000000)
    end MAIN;

    MAIN
end

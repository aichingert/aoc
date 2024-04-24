SET SERVEROUTPUT ON 

DECLARE
    

CREATE OR REPLACE FUNCTION partOne
    RETURN NUMBER 
AS
    a NUMBER(20);
    b NUMBER(20);
    counter NUMBER(5);
    fa      NUMBER(20) := 16807;
    fb      NUMBER(20) := 48271;
    modulos NUMBER(20) := 2147483647;
    partone NUMBER(5) := 0;
BEGIN
    a := 65;
    b := 8921;

    FOR _ in 1 .. 40000000 LOOP
        IF ( BITAND(a, 65536) = BITAND(b, 65536) ) THEN
            partone := partone + 1;
        END IF;

        a := MOD(a * fa, modulos);
        b := MOD(b * fb, modulos);
    END LOOP;

    return partone;
END;
/

CREATE OR REPLACE FUNCTION partTwo
RETURN NUMBER IS
    parttwo NUMBER(5) := 0;
BEGIN
    a := 65;
    b := 8921;

    WHILE counter < 5000000 LOOP
        WHILE MOD(a, 4) != 0 LOOP
            a := MOD(a * fa, modulos);
        END LOOP;

        WHILE MOD(b, 8) != 0 LOOP
            b := MOD(b * fb, modulos);
        END LOOP;

        IF ( BITAND(a, 65536) = BITAND(b, 65536) ) THEN
            parttwo := parttwo + 1;
        END IF;

        counter := counter + 1;
    END LOOP;

    return parttwo;
END;
/

CREATE OR REPLACE PROCEDURE Day12
AS
    a       NUMBER(20);
    b       NUMBER(20);
    t       BINARY_INTEGER := 65536;
    t2      BINARY_INTEGER := 0;
    i       NUMBER(20) := 0;
    fa      NUMBER(20) := 16807;
    fb      NUMBER(20) := 48271;
    modulos NUMBER(20) := 2147483647;
    counter NUMBER(20) := 0;
    partone NUMBER(20) := 0;
    parttwo NUMBER(20) := 0;
BEGIN
    a := 65;
    b := 8921;

    FOR i in 1 .. 10 LOOP
        t2 := a;

        DBMS_OUTPUT.PUT_LINE('t2: ' || t2 || ' ' || BITAND(t2, t));
        DBMS_OUTPUT.PUT_LINE('loa: ' || BITAND(a, 65536) || ' a ' || a);
        DBMS_OUTPUT.PUT_LINE('lob: ' || BITAND(b, 65536) || ' b ' || b);

        IF ( BITAND(a, 65536) = BITAND(b, 65536) ) THEN
            partone := partone + 1;
        END IF;

        a := MOD(a * fa, modulos);
        b := MOD(b * fb, modulos);
    END LOOP;

    DBMS_OUTPUT.PUT_LINE('Part one: ' || partone);
END;
/

SET SERVEROUTPUT ON 

DECLARE
    cur NUMBER(5);
    inp VARCHAR2(4000);
BEGIN
    cur := 1;

    select data into inp from input where linenr = cur;

    DBMS_OUTPUT.PUT_LINE(inp);
END;
/

DROP TABLE INPUT;

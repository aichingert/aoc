SET SERVEROUTPUT ON 

CREATE TABLE INPUT (
    linenr NUMBER(5) GENERATED ALWAYS as IDENTITY NOT NULL,
    data VARCHAR2(4000) NOT NULL
);

CREATE SEQUENCE linenr_seq;

CREATE OR REPLACE TRIGGER linenr_increment 
BEFORE INSERT ON input
FOR EACH ROW

BEGIN
  SELECT linenr_seq.NEXTVAL
  INTO   :new.linenr
  FROM   dual;
END;
/


INSERT INTO INPUT VALUES('0 <-> 2');
INSERT INTO INPUT VALUES('0 <-> 2');
INSERT INTO INPUT VALUES('0 <-> 2');

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

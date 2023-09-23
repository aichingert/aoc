with Ada.Text_IO; use Ada.Text_IO;
with Ada.Containers.Vectors;

procedure Day_04 is
	F	   : File_Type;
	File_Name  : constant String := "../input/04";

	Answer_One : Integer := 0;
	Answer_Two : Integer := 0;


	function Part_One (Line : String) return Integer
	is
		fs  	 : Natural := 1;
		fe 	 : Natural := 1;
		ss 	 : Natural := 1;
		se 	 : Natural := 1;
	begin
		for I in Line'Range loop
			if Line (I) = ' ' then
				fe := I - 1;
				ss := I + 1;

				for J in I + 1 .. Line'Last loop
					if Line (J) = ' ' then
						se := J - 1;

						if Line (fs .. fe) = Line (ss .. se) then 
							return 0;
						end if;
						ss := J + 1;
					end if;
				end loop;

				if fs /= ss and then Line (fs .. fe) = Line(ss .. Line'Last) then
					return 0;
				end if;

				fs:= I + 1;
			end if;
		end loop;

		return 1;
	end Part_One;

	function Part_Two (Line : String) return Integer
	is
		This	   : Integer := 0;
		Other	   : Integer := 0;
		Last_Start : Natural := 0;
	begin
		for I in Line'Range loop
			if Line (I) = ' ' then
				for J in I + 1 .. Line'Last loop
					if Line (J) = ' ' then
						if This = Other then
							return 0;
						end if;

						Other := 0;
						Last_Start := J + 1;
					else
						Other := Other + Character'Pos (Line (J));
					end if;
				end loop;

				if I + 1 /= Last_Start and then This = Other then
					return 0;
				end if;

				This := 0;
				Other := 0;
			else
				This := This + Character'Pos (Line (I));
			end if;
		end loop;

		return 1;
	end Part_Two;
begin
	Open (F, In_File, File_Name);

	while not End_Of_File (F) loop
		declare
			Line : String := Get_Line (F);
		begin
			Answer_One := Answer_One + Part_One (Line);
			Answer_Two := Answer_Two + Part_Two (Line);
		end;
	end loop;

	Put_Line ("Part one: " & Integer'Image (Answer_One));
	Put_Line ("Part two: " & Integer'Image (Answer_Two));
	Close (F);
end Day_04;


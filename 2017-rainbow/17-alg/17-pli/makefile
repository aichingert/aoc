# Makefile for PL/I Sample Programs
# Iron Spring PL/I 1.2.0 [Linux]

# ******************************************************
# ***** This file builds sample executables linked *****
# ***** with the static library.                   *****
# ***** May be run on either 32-bit or 64-bit Linux*****
# ******************************************************

PLI	= plic
PLIFLGS	= -lsiaxgo -ew "-cn(^) -ilib/include"
INC     = lib/include
LIBDIR	= lib/static

ALL = 17
	
all: ${ALL}	

clean:	
	rm ${ALL} *.obj *.exe *.o *.lst *.map

%.o:	%.pli 
	${PLI} -C -dELF ${PLIFLGS}  $^ -o $*.o
%:	%.o
	ld -z muldefs -Bstatic -M -o $@		\
	--oformat=elf32-i386			\
	-melf_i386                              \
	-e main  				\
	 $@.o 					\
 	--start-group   			\
	  /lib/libprf.a 			\
 	--end-group 				\
	>$@.map

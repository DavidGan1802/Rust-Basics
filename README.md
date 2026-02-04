# My Software Exercises for CoE 164 (Computing Platforms)

## **W01C_GAN.rs**

Create a system that converts the numerical amount into Filipino currency in words.

### Input:

The input starts with a number 𝑇 on a single line denoting the number of remittance forms.
𝑇 lines then follow, with each line denoting the numeral amount written on the form. Each
line consists of two space-separated nonnegative integers 𝑝 𝑐 denoting the whole Pesos
and Centavos, respectively.

### Output:

The output consists of 𝑇 lines, with each line denoting the corresponding form in the input.
Each line should be of the format 𝐶𝑎𝑠𝑒 #𝑡: 𝑊 where 𝑡 is the serial of the test case starting
from 1 and 𝑊 the amount written in Filipino. The amount should be written in all lowercase
letters.

### Constraints:

Both 𝑝 and 𝑐 can never be zero.

Assume that all of the inputs are well-formed and are always provided within these constraints.
It is not required to handle any errors.

## **W02B_GAN.rs**

Create a record finder module wherein a user will enter a student number and then it will return
the name and degree of the student.

### Input:

The input starts with a number *S* on a single line denoting the number of students to save 
into the module. *S* lines then follow, with each block of three lines denoting a student. 
The first line of the block contains the 9-digit student number, the second line contains the
name of the student, and the third line contains the degree.

After the block of student information is a list of commands. The first line of this command
block contains a number *C* denoting the number of commands that will follow. *C* lines then 
follow, denoting a command. A command can be written in any of the two formats.

- g (sn) - fetch a record with the given student number sn.
- u (sn) (update_cmd) - update a record with the given student number sn.
  - update_cmd can be only be from one of the following command:
    - d (degree) - update the degree of the student in the record degree 

### Output:

- g(sn)
  - If sn exists in the records:
    - >Get (sn)\
      >Name: (name)\
      >Degree: (degree)
  - If sn does not exist in the records:
    - >Get (sn)\
      >Does not exist

- u(sn)(update_cmd)
  - If sn exists in the records:
    - >Update (sn) (update_cmd)\
      >Success  
  - If sn doesn't exist in the records
    - >Update (sn) (update_cmd)\
      >Does not exist
  - update_cmd should be one of the following depending on the command received:
    - >d - Degree 

### Constraints:

0 < *S*, *C* < 101

sn always contains nine digits

Assume that all of the inputs are well-formed and are always provided within these constraints.
It is not required to handle any errors.

You are required to create and use a struct named StudentRecords with fields name:
String, degree: String, and sn: u32.

## **W03C_GAN.rs**

For this module, make data structures handling the RGB and HSV color values and make routines
that convert from one color model to another. The module will also have routines that determine
the chromatic, complementary, and triad colors relative to a single color.

The inputs and outputs are in w03_tests.rs

*Credits: Carl C. Dizon and Nestor Michael C. Tiglao*

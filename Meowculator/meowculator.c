#include <stdio.h>
//  All operating systems sucks, but Linux just sucks less
int main() {
    int x, y;
    int choice;
    int result;
    // "Avoiding complexity reduces bugs." Sir ill use inline ASM all i want!
    printf("Enter the first number: ");
    scanf("%d", &x);

    printf("Enter the second number: ");
    scanf("%d", &y);

    printf("Select operation:\n");
    printf("1. Addition\n");
    printf("2. Subtraction\n");
    printf("3. Multiplication\n");
    printf("4. Division\n");
    printf("Enter your choice (1-4): ");
    scanf("%d", &choice); // Message to future self:
    // 01101011 01101001 01101100 01101100 00100000 01111001 01101111 01110101 01110010 01110011 01100101 01101100 01100110

    // Bad code but as a famous finish guy once said: Bad programmers worry about the code. Good programmers worry about data structures and their relationships.
    switch (choice) {
        case 1: // Addition
            __asm__(
                "addl %1, %0;"    // Add y (operand 1) to x (operand 0) im mentally unstable
                : "+r" (x)        // Output operand: x, "+" denotes read/write yep pretty much
                : "r" (y)         // Input operand: y i hate my life
            );
            result = x;
            break;
        case 2: // Subtraction
            __asm__(
                "subl %1, %0;"    // Subtract y (operand 1) from x (operand 0)! no shit i can see the code!
                : "+r" (x)        // Output operand: x, "+" denotes read/write, i got promoted to customer!
                : "r" (y)         // Input operand: y, I WONDER !!!
            );
            result = x;
            break;
        case 3: // Multiplication (attempt 3)
            __asm__(
                "imull %1, %0;"   // Multiply x (operand 0) by y (operand 1) // sublime text
                : "=r" (result)   // Output operand: result, "=" denotes write-only like a bitch
                : "0" (x), "r" (y) // x and y, wanan go to xy: i wasted your time reading this it isnt funny at all no joke fuck off
            );
            break;
        case 4: // ASM Sucks at division
            if (y != 0) {
                result = x / y;
            } else {
                printf("Error: Division by zero\n"); // fuck you
                return 1;
            }
            break;
        default:
            printf("Invalid choice.\n");
            return 1;
    }

    printf("Result: %d\n", result);

    return 0; //Software is like sex: It's better when it's free.
}

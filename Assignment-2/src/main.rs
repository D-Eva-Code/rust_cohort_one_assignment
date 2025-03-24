fn main() {
    //An array of scores of 10 students
    let student_record: [u32; 10]= [45,58,70,93,34,55,46,84,61,59]; //created an array of 10 student's scores
    let total: u32= student_record.iter().sum();//calculate the sum of the scores
    println!("Total score is {}", total); //print the sum of the scores
    let average: f32= total as f32 /student_record.len() as f32;//calculate the average of the scores
    println!("Average of the scores is {}", average);//prints out the average of the scores
    let mut index=1;
    let mut highest_score= student_record[0];//assign the highest score to the first score in the array
    let mut fail=0;
    let mut failed= Vec::new();//create an empty vector or array to store the failed scores
    let mut passed = Vec::new();//create an empty vector or array to store the passed scoress
    for a in student_record{//loop through the student_record array
        if a > 50 {
            passed.push(a);//store the passed scores in the empty "passed" array we initially created
        }
        if a > highest_score  {  
            highest_score= a;
            index+=1;
        }
        if a < 50{
            fail +=1;//increment the "fail" to get the number of students that failed
            failed.push(a);//store the failed scores in the empty "failed" array we initially created
        }     
    }
    println!("The student with the highest score is with score of {}",highest_score);// prints score of student with the highest score
    println!("{} students failed",fail);// prints number of students that failed
    println!("Students with scores {:?} failed and students with scores {:?} passed.", failed, passed);// prints scores of students that failed and passed
} 


void setup() {
  Serial.begin(4800);

}

void loop() {
  // put your main code here, to run repeatedly:
  int sensorValue = analogRead(A0);
  
  Serial.println(sensorValue);
  delay(30);
}

//#include <cstdio>
// class MyClass {
// public:
//     MyClass();
//     int retint(int i);
//     MyClass factory();
// };
// MyClass::MyClass() {}
// int MyClass::retint(int i) {}
// MyClass MyClass::factory() {}
// #include <stdexcept>

// namespace MySpace {
//     class MyClass {
//         //public:
//         ~MyClass();
//     };
//     MyClass::~MyClass(){}
//     // class SC {
//     //     MyClass var;
//     //     public:
//     //     //~SC();
//     //     int member();
//     // };
//     // //SC::~SC(){}
//     // int SC::member(){}
// }

class MyClass {
  //void privmember();
  public:
  void pubmember();
};

// class Outer {
//   private:
//   class Priv {};
//   public:
//   class Pub {};
// };

// struct NormalStruct {};
// typedef struct {} DefinedStruct;
// DefinedStruct NormalStruct();
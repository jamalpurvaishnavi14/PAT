rules_version = '2';
service cloud.firestore {
  match /databases/{database}/documents {
    match /users/{document} {
      allow create: if request.auth.uid == document;
      allow read: if true;
      allow write: if request.auth.uid == document;
      allow delete: if false;
    }

    match /appointments/{document} {
      allow create: if true;
      allow read: if true;
      allow write: if false;
      allow delete: if false;
    }

    match /asdfasdf/{document} {
      allow create: if true;
      allow read: if true;
      allow write: if false;
      allow delete: if false;
    }
  }
}

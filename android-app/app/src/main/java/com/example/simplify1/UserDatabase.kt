package com.example.simplify1

import androidx.room.Database
import androidx.room.RoomDatabase

@Database(entities = [Users :: class], version = 1)
abstract class UserDatabase : RoomDatabase() {
    abstract fun userDataDao() : UserDataDao
}
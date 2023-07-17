/*
 * Copyright © 2023 Blue Habit.
 *
 * Unauthorized copying, publishing of this file, via any medium is strictly prohibited
 * Proprietary and confidential
 */

package com.bluehabit.budgetku.component.user.repo;

import com.bluehabit.budgetku.component.user.entity.UserProfile;
import org.springframework.data.repository.CrudRepository;
import org.springframework.data.repository.PagingAndSortingRepository;

public interface UserProfileRepository extends PagingAndSortingRepository<UserProfile, String>, CrudRepository<UserProfile, String> {
//    @Query("SELECT U FROM UserProfile  U WHERE U.deleted=false")
//    Page<UserProfile> getAllUsers(Pageable pageable);
}
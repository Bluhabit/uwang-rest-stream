package com.bluehabit.budgetku.model

data class PagingDataResponse<OUTPUT>(
    var page:Int,
    var size:Int,
    var totalPages:Int,
    var totalData:Long,
    var items:List<OUTPUT>
)
